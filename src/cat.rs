extern crate getopts;
use getopts::{optflag,getopts,Matches};
use std::{os,io};
use std::io::BufferedReader;

fn handle_line(line: &str, line_number: int, opts: &Matches) {
    if opts.opt_present("n") {
        print!("{:6d}  {}", line_number, line);
    } else {
        print!("{}", line);
    }
}

fn main() {
    let opts = [
        optflag("n", "number", "number all output lines"),
    ];
    let mut args = os::args();

    // cat only stdin if no args
    if args.len() == 1 {
        args = ~[~"", ~"-"];
    }
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => m,
        Err(f) => fail!(f.to_err_msg()),
    };

    for name in matches.free.iter() {
        let filename = if name == &~"-" { ~"/dev/stdin" } else { name.clone() };
        match io::File::open(&Path::new(filename.clone())) {
            Err(e) => println!("cat: cannot open {}: {}", filename, e),
            Ok(file) => {
                let mut line_number = 1;
                for r in BufferedReader::new(file).lines() {
                    match r {
                        Err(e) => {
                            println!("cat: cannot read from {}: {}", filename, e);
                            break;
                        }
                        Ok(line) => handle_line(line, line_number, &matches),
                    }
                    line_number += 1;
                }
            }
        }
    }
}
