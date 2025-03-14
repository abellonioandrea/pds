// paste this file into main.rs
use std::env::args;
use std::fs::File;
use std::io::Read;

fn stats(text: &str) -> [u32; 26] {
    let mut contatore = [0; 26];
    for c in text.chars() {
        let indice: isize = (c.to_ascii_lowercase() as isize) - 97;
        if(indice >= 0 && indice < 26) {
            contatore[indice as usize] += 1;
        }
    }
    contatore
}

fn is_pangram(counts: &[u32]) -> bool {
    if(counts.len()==26) {
        let mut check: bool = true;
        for valore in counts {
            if (*valore == 0) {
                check = false;
            }
        }
        check
    }
    else{
        false
    }
}

// call this function from main
// load here the contents of the file
pub fn run_pangram() {
    let args: Vec<String> = args().skip(1).collect();
    if (args.len() > 0) {
        let mut file = File::open(args[0].clone()).unwrap();
        let mut testo = String::new();
        file.read_to_string(&mut testo);
        let valori = stats(&testo);
        let mut check:bool = true;
        for valore in valori {
            if (valore == 0){
                check = false;
            }
        }
        if(check){
            println!("String is a pangram");
        }
        else{
            println!("String is not a pangram");
        }
    }
}


// please note, code has been splittend in simple functions in order to make testing easier

#[cfg(test)] // this is a test module
mod tests
{
    // tests are separated modules, yuou must import the code you are testing
    use super::*;

    #[test]
    fn test_all_ones() {
        let counts = [1; 26];
        assert!(is_pangram(&counts));
    }

    #[test]
    fn test_some_zeros() {
        let mut counts = [0; 26];
        counts[0] = 0;
        counts[1] = 0;
        assert!(!is_pangram(&counts));
    }

    #[test]
    fn test_increasing_counts() {
        let mut counts = [0; 26];
        for i in 0..26 {
            counts[i] = i as u32 + 1;
        }
        assert!(is_pangram(&counts));
    }

    #[test]
    fn test_wrong_size()  {
        let counts = [1; 25];
        assert!(!is_pangram(&counts));
    }

    #[test]
    fn test_stats_on_full_alphabet() {
        let counts = stats("abcdefghijklmnopqrstuvwxyz");
        for c in counts {
            assert!(c == 1);
        }
    }

    #[test]
    fn test_stats_on_empty_string() {
        let counts = stats("");
        for c in counts {
            assert!(c == 0);
        }
    }

    #[test]
    fn test_stats_missing_char() {
        let counts = stats("abcdefghijklmnopqrstuvwxy");
        for c in counts.iter().take(25) {
            assert!(*c == 1);
        }
        assert!(counts[25] == 0);

    }

    #[test]
    fn test_stats_on_full_tring() {
        let contents = "The quick brown fox jumps over the lazy dog";
        let counts = stats(contents);
        for c in counts {
            assert!(c > 0);
        }
    }

    #[test]
    fn test_stats_with_punctuation() {
        let contents = "The quick brown fox jumps over the lazy dog!";
        let counts = stats(contents);
        for c in counts {
            assert!(c > 0);
        }
    }

    #[test]
    fn test_missing_char_on_full_string() {
        let contents = "The quick brown fox jumps over the laz* dog";
        let counts = stats(contents);
        println!("{:?}", counts);
        for (i, c) in counts.iter().enumerate() {
            if i == 24 {
                assert!(*c == 0);
            } else {
                assert!(*c > 0);
            }

        }
    }

    #[test]
    fn test_is_pangram() {
        let counts = stats("The quick brown fox jumps over the lazy dog");
        assert!(is_pangram(&counts));
    }
}

fn main() {
    run_pangram();
}
