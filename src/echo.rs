use std::{os};

fn main() {
    let mut first = true;
    let mut newline = true; /* add trailing newline? */
    let mut escapes = false; /* process escapes? */

    for arg in os::args().iter().skip(1) {
        if first {
            if arg == &~"-e" {
                escapes = true;
                continue;
            } else if arg == &~"-E" {
                escapes = false;
                continue;
            } else if arg == &~"-n" {
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
