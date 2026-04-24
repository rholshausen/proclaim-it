use proclaim_it::{assert_that, spec};

#[spec]
fn basic_assertions() {
    let x = 2 + 2;
    let greeting = "hello world";
    let result: Result<i32, &str> = Ok(42);

    assert_that! {
        x == 2
        greeting contains "worldx"
        result is Err
    }
}
