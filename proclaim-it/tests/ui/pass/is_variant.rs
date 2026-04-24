use proclaim_it::assert_that;

fn main() {
    let ok: Result<i32, &str> = Ok(1);
    let err: Result<i32, &str> = Err("oops");
    let some: Option<i32> = Some(42);

    assert_that! {
        ok is Ok
        err is Err
        some is Some
    }
}
