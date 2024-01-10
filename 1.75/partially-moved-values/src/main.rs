fn main() {
    let foo = (Box::new(27), Box::new(42));
    let x = foo.0;
    let y = foo.1;

    match foo {
        (_, _) => {
            println!("The sun of x and y is: {:?}", *x + *y);
        }
    }
}
