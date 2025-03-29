
#![feature(async_stream)]
use async_stream::stream;

use futures::{prelude::*, stream::poll_fn};
use std::task::Poll;
// First, the struct:


/// A stream which counts from one to five
struct Counter {
    count: usize,
}

// we want our count to start at one, so let's add a new() method to help.
// This isn't strictly necessary, but is convenient. Note that we start
// `count` at zero, we'll see why in `poll_next()`'s implementation below.
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// Then, we implement `Stream` for our `Counter`:

impl Stream for Counter {
    // we will be counting with usize
    type Item = usize;

    // poll_next() is the only required method
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // Increment our count. This is why we started at zero.
        self.count += 1;

        // Check to see if we've finished counting or not.
        if self.count < 6 {
            Poll::Ready(Some(self.count))
        } else {
            Poll::Ready(None)
        }
    }
}

#[tokio::main]
fn main(){
    let counter = Counter{ count: 0 };
    while let Some(value) = counter.next().await {
        println!("got {}", value);
    }

}
