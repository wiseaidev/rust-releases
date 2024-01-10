fn check_range(value: usize) {
    match value {
        ..=42 => println!("Value is 42 or smaller!"),
        43.. => println!("Value is greater than 42"),
    }
}

fn main() {
    let value = 42;

    check_range(value);
}
