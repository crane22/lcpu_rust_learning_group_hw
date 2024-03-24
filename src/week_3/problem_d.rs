// Problem D: Functions
pub fn quiz() {
    fn foo(x: i32, y: i32) -> &'static str {    // FIX ME
        println!("x: {}, y: {}", x, y);
        "hello"
    }

    assert_eq!(foo(1, 2), "hello");
}