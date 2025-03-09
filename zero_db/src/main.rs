use std::io::{self, Write};

fn main() {
    loop {
        let mut input = String::new();
        print!("db > ");
        io::stdout().flush().expect("Failed to flush to stdout");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");

        input = input.trim().to_string();

        match input.as_str() {
            "exit" => std::process::exit(1),
            _ => {
                println!("Unrecognized command: {}", input);
                continue;
            }
        }
    }
}
