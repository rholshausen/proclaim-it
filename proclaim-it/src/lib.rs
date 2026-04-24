pub use proclaim_it_macros::spectest;

#[cfg(test)]
mod tests {
    use super::*;

    #[spectest]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
