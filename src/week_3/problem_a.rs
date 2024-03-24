// Problem A: Mutable And Shadowing
pub fn quiz() {
    let mut x: i32 = 1; // FIX ME
    assert_eq!(x, 1);

    x = 2;
    assert_eq!(x, 2);

    let mut x = "hello".to_string(); // FIX ME
    x.push_str(", world!");
    assert_eq!(x, "hello, world!");
}