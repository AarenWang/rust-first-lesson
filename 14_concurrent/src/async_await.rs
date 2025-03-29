use futures::executor::block_on;
use std::future::Future;

async fn do_something() {
    println!("HaHa")
}

fn main() {
    let future = do_something();
    block_on(future);
}
