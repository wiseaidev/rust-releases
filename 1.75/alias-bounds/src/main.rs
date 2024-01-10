// Example trait with a region parameter
trait Captures<'a> {}
impl<T> Captures<'_> for T {}

// Opaque function returning an impl Trait with region capture
fn capture<'o>(_: &'o mut ()) -> impl Sized + Captures<'o> + 'static {
    // Implementation details
    todo!()
}

// Function using the opaque type with a borrow checker issue
fn test_two_mut(mut x: ()) {
    let _f1 = capture(&mut x);
    let _f2 = capture(&mut x);
    // In the fixed version, the liveness analysis considers 'static bound
    // and avoids borrow checker errors in certain cases.
    //~^ ERROR cannot borrow `x` as mutable more than once at a time
}

fn main() {
    test_two_mut(());
}
