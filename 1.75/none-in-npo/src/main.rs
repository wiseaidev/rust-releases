use std::mem::{size_of, transmute};

struct MyType {
    _data: u32,
}

fn main() {
    let option_transmuted: Option<MyType> = unsafe { transmute(&[0u8; size_of::<MyType>()]) };

    if option_transmuted.is_none() {
        println!("Transmutation successful: Option<MyType>::None produced");
    }
}