#![feature(std_misc)]
#![feature(test)]

extern crate test;

use std::time::duration::Duration;
use std::old_io::timer;

use test::Bencher;

pub fn sleep_ns (ns: i64) {
    let interval = Duration::nanoseconds(ns);
    timer::sleep(interval);
}

#[test]
fn it_works() {
    sleep_ns(1);
}

#[bench]
fn bench_foo (b: &mut test::Bencher) {
    b.iter(|| {
        sleep_ns(1);
    });
}
