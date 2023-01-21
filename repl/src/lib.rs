use std::io::{stdin, stdout, Write};

const PROMPT: &str = ">> ";

pub fn start() {
    loop {
        print!("{PROMPT}");
        stdout().flush().expect("Failed to flush line");

        let mut line = String::new();

        stdin().read_line(&mut line).expect("Failed to read line");

        let result = match wervc::parse(&line) {
            Ok(o) => o,
            Err(e) => {
                println!("{:?}", e);
                continue;
            }
        };

        println!("{result}");
    }
}
