extern crate para_bm;

use std::time::Duration;

use para_bm::{sleep_ns};

fn fast () {
    sleep_ns(1000);
}

fn slow () {
    sleep_ns(1000000);
}

fn my_bench<F> (f: F) where F: Fn() {

    // first "throw away" run
    let mut d = Duration::span(|| {
        f();
    });
    let ns = d.num_nanoseconds().unwrap();
    println!("num nano seconds first run {}", ns);


    // iterate to get more cache hits, etc...
    for _ in 0 .. 10 {
        d = Duration::span(|| {
            f();
        });
        let ns = d.num_nanoseconds().unwrap();
        println!("num nano seconds {}", ns);
    }
}

fn main () {


    my_bench(||{
        slow()
    });

    my_bench(||{
        fast()
    });

}
