pub use proclaim_it_macros::assert_that;
pub use proclaim_it_macros::spec;

#[cfg(test)]
mod tests {
    use super::*;

    #[spec]
    fn basic_assertions() {
        let x = 2 + 2;
        let greeting = "hello world";
        let result: Result<i32, &str> = Ok(42);

        assert_that! {
            x == 4
            greeting contains "world"
            result is Ok
        }
    }
}
