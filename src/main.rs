use std::io::Read;

fn main() {
    let mut args = std::env::args();

    if let Some(path) = args.nth(1) {
        let mut file = std::fs::File::open(path).unwrap();
        let mut input = String::new();

        file.read_to_string(&mut input).unwrap();

        let result = wervc::Compiler::new().parse(&input).unwrap();

        println!("{}", result);
        return;
    }

    repl::start();
}
