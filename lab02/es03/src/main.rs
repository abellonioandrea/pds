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
    match e {
        Error::Simple(t)=>{
            println!("Simple error at {}", t.elapsed().unwrap().as_nanos());
        }
        Error::Complex(t, s)=>{println!("Complex error at {} {}", t.elapsed().unwrap().as_nanos(), s);}
    }
}

fn manageFile(){
    match fs::read_to_string("test.txt") {
        Ok(text) => {
            println!("{:?}", text);
            if let Err(e) = fs::write("test.txt", text.repeat(10)) {
                println!("Failed to write to file: {}", e);
            }
        }
        Err(e) => {
            println!("Failed to read file: {}", e);
        }
    }
}
