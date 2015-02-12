extern crate para_bm;

use std::time::Duration;

use para_bm::{sleep_ns};

fn main () {

    // first "throw away" run
    let mut d = Duration::span(|| {
        sleep_ns(1);
    });
    let ns = d.num_nanoseconds().unwrap();
    println!("num nano seconds first run {}", ns);


    // iterate to get more cache hits, etc...
    for _ in 0 .. 10 {
        d = Duration::span(|| {
            sleep_ns(1);
        });
        let ns = d.num_nanoseconds().unwrap();
        println!("num nano seconds {}", ns);
    }


}
