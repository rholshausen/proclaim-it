use std::fmt;

pub use proclaim_it_macros::assert_that;
pub use proclaim_it_macros::spec;

// ── message formatters (called from generated code via ::proclaim_it::*) ─────
#[doc(hidden)]
pub fn format_error(failures: Vec<String>) -> String {
  let mut s = format!("proclaim-it: {} assertion(s) failed:\n\n", failures.len());
  for (index, f) in failures.iter().enumerate() {
    let mut lines = f.lines();
    let first = lines.next().unwrap();
    s.push_str(format!("  {}): {}", index + 1, first).as_str());

    while let Some(line) = lines.next() {
      s.push_str(format!("\n      {}", line).as_str());
    }

    s.push('\n');
    s.push('\n');
  }
  s
}

#[doc(hidden)]
pub fn format_eq_failure<L, R>(assertion: &str, left: &L, right: &R) -> String
where
  L: fmt::Debug + PartialEq<R>,
  R: fmt::Debug,
{
  format!(
    "assertion `{}` failed\n{}",
    assertion,
    pretty_assertions::Comparison::new(left, right)
  )
}

#[doc(hidden)]
pub fn format_ne_failure(assertion: &str, value: &dyn fmt::Debug) -> String {
  format!("assertion `{}` failed\n  value: {:?}", assertion, value)
}

#[doc(hidden)]
pub fn format_is_failure(assertion: &str, actual: &dyn fmt::Debug) -> String {
  format!("{}\n  actual: {:?}", assertion, actual)
}

#[doc(hidden)]
pub fn format_contains_failure(
  assertion: &str,
  subject: &dyn fmt::Debug,
  pattern: &dyn fmt::Debug,
) -> String {
  format!(
    "{}\n  subject: {:?}\n  pattern: {:?}",
    assertion, subject, pattern
  )
}

#[doc(hidden)]
pub fn format_ord_failure(
  assertion: &str,
  left: &dyn fmt::Debug,
  right: &dyn fmt::Debug,
) -> String {
  format!("{}\n   left: {:?}\n  right: {:?}", assertion, left, right)
}

