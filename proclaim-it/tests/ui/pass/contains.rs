use proclaim_it::assert_that;

fn main() {
    let email = "alice@example.com";
    let greeting = "hello world";

    assert_that! {
        email contains "@"
        greeting contains "hello"
    }
}
