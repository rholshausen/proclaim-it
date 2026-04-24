use proc_macro::TokenStream;
use proc_macro2::{Spacing, TokenStream as TokenStream2, TokenTree};
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// Marks a function as a proclaim-it spec test, enabling enhanced assertion reporting.
#[proc_macro_attribute]
pub fn spec(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    let block = &input.block;
    let attrs = &input.attrs;

    let expanded = quote! {
        #[test]
        #(#attrs)*
        fn #name() {
            #block
        }
    };

    expanded.into()
}

/// Declarative multi-assertion block. Each line is one assertion:
///
/// ```ignore
/// assert_that! {
///     x == 42
///     name contains "alice"
///     result is Ok
///     count > 0
/// }
/// ```
///
/// Supported operators: `==`, `!=`, `<`, `<=`, `>`, `>=`, `contains`, `is`
///
/// `contains` calls `.contains()` on the left-hand side.
/// `is` uses `matches!` to check an enum variant; bare paths like `Ok` expand to `Ok(..)`.
#[proc_macro]
pub fn assert_that(input: TokenStream) -> TokenStream {
    let tokens: Vec<TokenTree> = TokenStream2::from(input).into_iter().collect();
    let lines = split_into_lines(&tokens);

    let mut output = TokenStream2::new();
    for line in lines {
        if !line.is_empty() {
            output.extend(generate_assertion(&line));
        }
    }

    output.into()
}

// ── helpers ──────────────────────────────────────────────────────────────────

fn split_into_lines(tokens: &[TokenTree]) -> Vec<Vec<TokenTree>> {
    let mut lines: Vec<Vec<TokenTree>> = Vec::new();
    let mut current: Vec<TokenTree> = Vec::new();
    let mut prev_line: Option<usize> = None;

    for token in tokens {
        let line = token.span().start().line;
        if let Some(prev) = prev_line {
            if line > prev && !current.is_empty() {
                lines.push(std::mem::take(&mut current));
            }
        }
        prev_line = Some(line);
        current.push(token.clone());
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
}

enum Op {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Contains,
    Is,
}

/// Returns `(op, op_start, op_end)` where `tokens[..op_start]` is the LHS
/// and `tokens[op_end..]` is the RHS.
fn find_op(tokens: &[TokenTree]) -> Option<(Op, usize, usize)> {
    let mut i = 0;
    while i < tokens.len() {
        match &tokens[i] {
            TokenTree::Ident(ident) => {
                // Skip identifiers that are part of a method call (preceded by `.`)
                let preceded_by_dot = i > 0
                    && matches!(&tokens[i - 1], TokenTree::Punct(p) if p.as_char() == '.');
                if !preceded_by_dot {
                    match ident.to_string().as_str() {
                        "contains" => return Some((Op::Contains, i, i + 1)),
                        "is" => return Some((Op::Is, i, i + 1)),
                        _ => {}
                    }
                }
            }
            TokenTree::Punct(p) => {
                let next_ch = tokens.get(i + 1).and_then(|t| match t {
                    TokenTree::Punct(p2) => Some(p2.as_char()),
                    _ => None,
                });
                // Two-character operators must be checked before single-character ones
                match (p.as_char(), p.spacing(), next_ch) {
                    ('=', Spacing::Joint, Some('=')) => return Some((Op::Eq, i, i + 2)),
                    ('!', Spacing::Joint, Some('=')) => return Some((Op::Ne, i, i + 2)),
                    ('<', Spacing::Joint, Some('=')) => return Some((Op::Le, i, i + 2)),
                    ('>', Spacing::Joint, Some('=')) => return Some((Op::Ge, i, i + 2)),
                    ('<', _, _) => return Some((Op::Lt, i, i + 1)),
                    ('>', _, _) => return Some((Op::Gt, i, i + 1)),
                    _ => {}
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}

fn to_stream(tokens: &[TokenTree]) -> TokenStream2 {
    tokens.iter().cloned().collect()
}

fn to_display(tokens: &[TokenTree]) -> String {
    to_stream(tokens).to_string()
}

fn generate_assertion(tokens: &[TokenTree]) -> TokenStream2 {
    match find_op(tokens) {
        Some((Op::Eq, s, e)) => {
            let lhs = to_stream(&tokens[..s]);
            let rhs = to_stream(&tokens[e..]);
            let msg = format!("{} == {}", to_display(&tokens[..s]), to_display(&tokens[e..]));
            quote! { ::pretty_assertions::assert_eq!(#lhs, #rhs, "{}", #msg); }
        }
        Some((Op::Ne, s, e)) => {
            let lhs = to_stream(&tokens[..s]);
            let rhs = to_stream(&tokens[e..]);
            let msg = format!("{} != {}", to_display(&tokens[..s]), to_display(&tokens[e..]));
            quote! { ::pretty_assertions::assert_ne!(#lhs, #rhs, "{}", #msg); }
        }
        Some((Op::Contains, s, e)) => {
            let subject = to_stream(&tokens[..s]);
            let pattern = to_stream(&tokens[e..]);
            let msg = format!(
                "{} contains {}",
                to_display(&tokens[..s]),
                to_display(&tokens[e..])
            );
            quote! { ::core::assert!(#subject.contains(#pattern), "{}", #msg); }
        }
        Some((Op::Is, s, e)) => {
            let subject = to_stream(&tokens[..s]);
            let variant_tokens = &tokens[e..];
            let msg = format!(
                "{} is {}",
                to_display(&tokens[..s]),
                to_display(variant_tokens)
            );
            let variant = to_stream(variant_tokens);
            // If the RHS already contains parens/braces it's a full pattern; use as-is.
            // Otherwise append `(..)` so bare paths like `Ok` match any contents.
            let has_group = variant_tokens.iter().any(|t| matches!(t, TokenTree::Group(_)));
            if has_group {
                quote! { ::core::assert!(::core::matches!(#subject, #variant), "{}", #msg); }
            } else {
                quote! { ::core::assert!(::core::matches!(#subject, #variant(..)), "{}", #msg); }
            }
        }
        Some((Op::Lt, s, e)) => {
            let lhs = to_stream(&tokens[..s]);
            let rhs = to_stream(&tokens[e..]);
            let msg = format!("{} < {}", to_display(&tokens[..s]), to_display(&tokens[e..]));
            quote! { ::core::assert!(#lhs < #rhs, "{}", #msg); }
        }
        Some((Op::Le, s, e)) => {
            let lhs = to_stream(&tokens[..s]);
            let rhs = to_stream(&tokens[e..]);
            let msg = format!("{} <= {}", to_display(&tokens[..s]), to_display(&tokens[e..]));
            quote! { ::core::assert!(#lhs <= #rhs, "{}", #msg); }
        }
        Some((Op::Gt, s, e)) => {
            let lhs = to_stream(&tokens[..s]);
            let rhs = to_stream(&tokens[e..]);
            let msg = format!("{} > {}", to_display(&tokens[..s]), to_display(&tokens[e..]));
            quote! { ::core::assert!(#lhs > #rhs, "{}", #msg); }
        }
        Some((Op::Ge, s, e)) => {
            let lhs = to_stream(&tokens[..s]);
            let rhs = to_stream(&tokens[e..]);
            let msg = format!("{} >= {}", to_display(&tokens[..s]), to_display(&tokens[e..]));
            quote! { ::core::assert!(#lhs >= #rhs, "{}", #msg); }
        }
        None => {
            let src = to_display(tokens);
            let err = format!("proclaim-it: cannot parse assertion: `{src}`");
            quote! { compile_error!(#err); }
        }
    }
}
