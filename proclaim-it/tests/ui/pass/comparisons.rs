use proclaim_it::assert_that;

fn main() {
    let age = 30i32;

    assert_that! {
        age > 0
        age >= 30
        age < 100
        age <= 30
    }
}
