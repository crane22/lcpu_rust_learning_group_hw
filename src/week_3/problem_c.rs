// Problem C: Data Types 2
pub fn quiz() {
    let mut result = String::new();

    // Your code here
    let mut s = read_line().trim().to_string();
    s.retain(|c| c != 'y'); 
    let x = parse_i32(&read_line());
    for _ in 0..x {
        result.push_str(&s);
    }
    println!("{}", result);
}

fn read_line() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer
}

fn parse_i32(s: &str) -> i32 {
    s.trim().parse::<i32>().unwrap()
}