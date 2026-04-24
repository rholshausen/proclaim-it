use proclaim_it::assert_that;

fn main() {
    let x = 42i32;
    let name = "alice";

    assert_that! {
        x != 0
        name != "bob"
    }
}
