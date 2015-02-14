extern crate para_bm;

use std::time::Duration;
use std::thread::Thread;


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

    let mut sum = 0;
    // iterate to get more cache hits, etc...
    for _ in 0 .. 10 {
        d = Duration::span(|| {
            f();
        });
        let ns = d.num_nanoseconds().unwrap();
        sum = sum + ns;
    }

    let avg = (sum as f64) / 10.0;
    println!("average after first run {}", avg);
}

fn main () {


    println!("Running Threaded slow");
    my_bench(||{
        Thread::scoped(move || {
            slow();
        });
    });

    println!("Running Non-Threaded fast");
    my_bench(||{
        fast()
    });

}
