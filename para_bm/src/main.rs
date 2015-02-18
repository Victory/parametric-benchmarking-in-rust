#![feature(std_misc)]
#![feature(core)]

extern crate para_bm;

use std::time::Duration;
use std::thread::Thread;
use std::num::Float;

use para_bm::{sleep_ns};

fn fast () {
    sleep_ns(1000);
}

fn slow () {
    sleep_ns(10000000);
}

fn my_bench<F> (f: F) -> f64 where F: Fn() {

    // first "throw away" run
    let mut d = Duration::span(|| {
        f();
    });
    let ns = d.num_nanoseconds().unwrap();
    //println!("num nano seconds first run {}", ns);

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
    //println!("average after first run {}", avg);

    return avg;
}

fn main () {

    let mut num_ns = 10.0;

    for power in 0 .. 20 {
        println!("Size.....: {}", num_ns as i64);
        
        let t_avg = my_bench(||{
            let threads_holder = (0 .. 3).map(|_| {
                Thread::scoped(move || {
                    sleep_ns(num_ns as i64);
                })
            }).collect::<Vec<_>>();
        });
        println!("Runtime thread....: {}", t_avg);
        
        let s_avg = my_bench(||{
            let serial_holder = (0 .. 3).map(|_| {
                sleep_ns(num_ns as i64);
            }).collect::<Vec<_>>();
        });
        println!("Runtime serial....: {}", s_avg);
        
        num_ns *= 2.0;
    }
}
