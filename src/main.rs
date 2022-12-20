use std::{env, process::exit};

use werv::{io::execute_from_file, repl};

fn main() {
    let mut args = env::args();

    if let Some(c) = args.nth(1) {
        if c == "repl" {
            repl::start();
        } else {
            match execute_from_file(&c) {
                Err(e) => {
                    println!("{e}");
                    exit(1);
                }
                _ => {}
            }
        }
    }
}
