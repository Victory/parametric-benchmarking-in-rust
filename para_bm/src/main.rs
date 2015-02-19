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
    let iters = 100.0;
    for _ in 0 .. iters as usize {
        d = Duration::span(|| {
            f();
        });
        let ns = d.num_nanoseconds().unwrap();
        sum = sum + ns;
    }

    let avg = (sum as f64) / iters;
    //println!("average after first run {}", avg);

    return avg;
}

fn main () {


    let mut max_ns = 10000000.0;
    let mut min_ns = 0.0;
    //let mut half_ns;
    let mut qurt_ns;
    let mut num_ns = (max_ns + min_ns) / 2.0;

    for power in 0 .. 30 {
        println!("Size.....: {}", num_ns as i64);
        

        let t_avg = my_bench(||{
            let threads_holder = (0 .. 3).map(|_| {
                Thread::scoped(move || {
                    sleep_ns(num_ns as i64);
                })
            }).collect::<Vec<_>>();
        });
        //println!("Runtime thread....: {}", t_avg);
        
        let s_avg = my_bench(||{
            let serial_holder = (0 .. 3).map(|_| {
                sleep_ns(num_ns as i64);
            }).collect::<Vec<_>>();
        });
        //println!("Runtime serial....: {}", s_avg);
        
        println!("s - t .....: {}", s_avg - t_avg);
        


        if t_avg > s_avg {
            qurt_ns = (max_ns + num_ns) / 2.0;
            min_ns = num_ns;
            num_ns = qurt_ns;
        } else {
            qurt_ns = (min_ns + num_ns) / 2.0;
            max_ns = num_ns;
            num_ns = qurt_ns;
        }

        println!("qurt.....: {}", qurt_ns as i64);
        println!("jump.....: {}", max_ns - min_ns);

        println!("------------------------------------");

        if max_ns - min_ns < 1.0 {

            break;
        }


    }
}
