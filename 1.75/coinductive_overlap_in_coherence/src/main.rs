use std::cmp::Ordering;
use std::marker::PhantomData;

#[derive(PartialEq, Default)]
pub(crate) struct Interval<T>(PhantomData<T>);

impl<T, Q> PartialEq<Q> for Interval<T>
where
    Q: PartialOrd,
{
    fn eq(&self, _: &Q) -> bool {
        // For simplicity, let's assume equality if the underlying type T is comparable to Q
        true
    }
}

impl<T, Q> PartialOrd<Q> for Interval<T>
where
    Q: PartialOrd,
{
    fn partial_cmp(&self, _: &Q) -> Option<Ordering> {
        // For simplicity, let's assume the ordering is always equal for PhantomData<T>
        Some(Ordering::Equal)
    }
}

fn main() {
    let interval1 = Interval::<u32>::default();
    let interval2 = Interval::<u32>::default();

    // Now, you can use the implemented traits for comparisons.
    if interval1 == interval2 {
        println!("Intervals are equal!");
    }
}
