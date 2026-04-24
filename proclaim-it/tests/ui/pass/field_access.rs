use proclaim_it::assert_that;

struct User {
    name: String,
    age: u32,
}

fn main() {
    let user = User { name: "alice".to_string(), age: 30 };

    assert_that! {
        user.name == "alice"
        user.age > 0
        user.name contains "ali"
    }
}
