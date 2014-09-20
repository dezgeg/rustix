extern crate getopts;
use getopts::{optflag,getopts,Matches};
use std::{os,io};
use std::io::BufferedReader;

fn handle_line(line: &str, line_number: int, opts: &Matches) {
    if opts.opt_present("n") {
        print!("{:6d}\t{}", line_number, line);
    } else {
        print!("{}", line);
    }
}

fn main() {
    let opts = [
        optflag("n", "number", "number all output lines"),
    ];
    let matches = match getopts(os::args().tail(), opts) {
        Ok(m) => m,
        Err(f) => fail!(f.to_err_msg()),
    };

    // cat only stdin if no args
    let filenames = if matches.free.is_empty() {
        vec!("-".to_string())
    } else {
        matches.free.clone()
    };
    for name in filenames.iter() {
        let filename = if name.as_slice() == "-" { "/dev/stdin".to_string() } else { name.clone() };
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
                        Ok(line) => handle_line(line.as_slice(), line_number, &matches),
                    }
                    line_number += 1;
                }
            }
        }
    }
}
