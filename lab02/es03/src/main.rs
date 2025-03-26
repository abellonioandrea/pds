use std::fs;
use std::time::SystemTime;
use crate::Error::Complex;

enum Error{
    Simple(SystemTime),
    Complex(SystemTime, String)
}

fn main() {
    print_error(Error::Complex(SystemTime::now(), "time".to_string()));
}

fn print_error(e: Error){
    println!("{}", e[Complex]);
}

fn manageFile(){
    match fs::read_to_string("test.txt") {
        Ok(mut text) => {
            println!("{:?}", text);
            text.push_str("\n");
            let mut toPrint = String::new();
            for i in 0..10 {
                toPrint.push_str(&text.clone());
            }
            if let Err(e) = fs::write("test.txt", toPrint.to_string()) {
                println!("Failed to write to file: {}", e);
            }
        }
        Err(e) => {
            println!("Failed to read file: {}", e);
        }
    }
}
