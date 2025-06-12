use std::io::{self, Write};
mod eval;
fn main() {
    loop {
        print!(">> ");
        let _ = io::stdout().flush();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
            }
            Err(error) => println!("error: {error}"),
        }
        println!("{}",eval::eval(input));
    }
}