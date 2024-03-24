// Problem B: Data Types
pub fn quiz() {
    let x = read_i32();
    let y = read_f64();

    // Your code here
    let x_lowest_8_bits = (x & 0xFF) as u8;
    let y_integer_part = y as i32;
    let sum = x_lowest_8_bits as i32 + y_integer_part;
    println!("{}", x_lowest_8_bits);
    println!("{}", y_integer_part);
    println!("{}", sum);
}

fn read_i32() -> i32 {
    read()
}

fn read_f64() -> f64 {
    read()
}

fn read<T>() -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().parse::<T>().unwrap()
}