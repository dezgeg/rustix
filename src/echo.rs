use std::{os};

fn main() {
    let mut first = true;
    let mut newline = true; /* add trailing newline? */
    let mut escapes = false; /* process escapes? */

    for arg in os::args().iter().skip(1) {
        if first {
            if arg == &"-e".to_string() {
                escapes = true;
                continue;
            } else if arg == &"-E".to_string() {
                escapes = false;
                continue;
            } else if arg == &"-n".to_string() {
                newline = false;
                continue;
            }
        } else {
            print!(" ");
        }
        print!("{}", arg);
        first = false;
    }

    if newline {
        print!("\n");
    }
}
