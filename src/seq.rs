use std::{os};

fn do_seq(start: f64, incr: f64, end: f64) {
    // range_step_inclusive gives error: failed to find an implementation of trait std::num::CheckedAdd for f64
    let mut x = start;
    while x <= end {
        println!("{}", x);
        x += incr;
    }
}

fn main() {
    let args = os::args();
    // std::option::collect would be useful but it gives some error
    let nums = args.iter().skip(1).map(|s| match from_str::<f64>(s.clone()) {
        None => fail!("seq: not a number"),
        Some(x) => x,
    }).collect::<Vec<f64>>();

    match nums.as_slice() {
        [end] => do_seq(1.0, 1.0, end),
        [start, end] => do_seq(start, 1.0, end),
        [start, incr, end] => do_seq(start, incr, end),
        _ => println!("seq: wrong number of parameters"),
    }
}
