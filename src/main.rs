use std::io::Read;

fn main() {
    let mut args = std::env::args();

    if let Some(command) = args.nth(1) {
        match command.as_str() {
            "repl" => repl::start(),
            "run" => run(&mut args),
            path => {
                let mut file = std::fs::File::open(path).unwrap();
                let mut input = String::new();

                file.read_to_string(&mut input).unwrap();

                let mut compiler = wervc::Compiler::new();

                compiler.compile(input).unwrap();
                println!("{}", compiler.output);
            }
        }
    }
}

fn run(args: &mut std::env::Args) {
    if let Some(path) = args.next() {
        let mut file = std::fs::File::open(path).unwrap();
        let mut input = String::new();

        file.read_to_string(&mut input).unwrap();

        let result = wervc::Interpreter::new().run(&input).unwrap();

        println!("{}", result);
    } else {
        println!("No file provided");
    }
}
