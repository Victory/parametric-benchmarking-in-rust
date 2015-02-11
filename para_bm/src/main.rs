extern crate para_bm;

use std::time::Duration;

use para_bm::{sleep_ns};

fn main () {

    let d = Duration::span(|| {
        sleep_ns(3);
    });

    let ns = d.num_nanoseconds().unwrap();

    println!("num nano seconds {}", ns);
}
