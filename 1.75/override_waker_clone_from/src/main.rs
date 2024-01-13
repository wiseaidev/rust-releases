use futures::task::noop_waker;
use std::future::Future;
use std::pin::{pin, Pin};
use std::ptr;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

struct MyFuture {
    waker: Waker,
}

impl MyFuture {
    fn new() -> Self {
        Self {
            waker: noop_waker(),
        }
    }
}

impl Future for MyFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("With `clone`:");
        self.waker = cx.waker().clone();
        println!("waker is {:?}", self.waker);

        println!("With `clone_from`:");
        self.waker.clone_from(cx.waker());
        println!("waker is {:?}", self.waker);

        Poll::Ready(())
    }
}

fn main() {
    const RAW_WAKER: RawWaker = RawWaker::new(
        ptr::null_mut(),
        &RawWakerVTable::new(
            |_| {
                println!("cloning");
                RAW_WAKER
            },
            |_| {},
            |_| {},
            |_| {
                println!("dropping");
            },
        ),
    );
    let mut f = pin!(MyFuture::new());
    let waker = unsafe { Waker::from_raw(RAW_WAKER) };
    let mut cx = Context::from_waker(&waker);
    println!("First poll:");
    let _ = f.as_mut().poll(&mut cx);
    println!("Second poll:");
    let _ = f.as_mut().poll(&mut cx);
}