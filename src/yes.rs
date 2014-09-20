use std::os;
use std::str::StrVector;

fn main() {
    let args = os::args();
    let s = if args.len() == 1 { "y".to_string() } else { args.slice_from(1).connect(" ") };

    loop {
        println!("{}", s);
    }
}
