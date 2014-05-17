use std::{os,io};
use std::io::BufferedReader;

fn main() {
    let mut args = os::args();

    // cat only stdin if no args
    if args.len() == 1 {
        args = ~[~"", ~"-"];
    }

    for name in args.iter().skip(1) {
        let filename = if name == &~"-" { ~"/dev/stdin" } else { name.clone() };
        match io::File::open(&Path::new(filename.clone())) {
            Err(e) => println!("cat: cannot open {}: {}", filename, e),
            Ok(file) => {
                for r in BufferedReader::new(file).lines() {
                    match r {
                        Err(e) => {
                            println!("cat: cannot read from {}: {}", filename, e);
                            break;
                        }
                        Ok(line) => print!("{}", line),
                    }
                }
            }
        }
    }
}
