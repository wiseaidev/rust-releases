fn main() {
    let input = "a+++b";
    let pattern = "++";
    let mut split_inclusive = input.split_inclusive(pattern);
    println!("Forward: {:?}", split_inclusive.next());
    // This should return Some("+b") and not Some("b")
    println!("Backward: {:?}", split_inclusive.next_back());
}
