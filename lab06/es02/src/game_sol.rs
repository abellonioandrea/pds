use std::{thread, vec};

use itertools::{Itertools};

pub fn mk_ops(symbols: &[char], n: usize) -> Vec<String> {
    if n == 0 {
        return vec![String::new()];
    }

    let mut result = vec![];

    for &symbol in symbols {
        for perm in mk_ops(symbols, n - 1) {
            result.push(format!("{}{}", symbol, perm));
        }
    }

    result
}

pub fn prepare(s: &str) -> Vec<String> {
    let mut result = vec![];
    let ops = mk_ops(&['+', '-', '*', '/'], 4);

    for digit in s.chars().permutations(s.len()) {
        for op_seq in &ops {
            let mut s = String::new();
            let mut it_op = op_seq.chars();
            for d in digit.iter() {
                s.push(*d);
                if let Some(op) = it_op.next() {
                    s.push(op);
                }
            }
            result.push(s);
        }
    }
    result
}

pub fn verify(v: &[String]) -> Vec<String> {
    let mut results = vec![];
    for sol in v {
        let mut chars = sol.chars();
        let mut val = chars.next().unwrap().to_digit(10).unwrap() as i32;

        let mut valid = true;
        loop {
            match chars.next() {
                Some('+') => {
                    val += chars.next().unwrap().to_digit(10).unwrap() as i32;
                }
                Some('-') => {
                    val -= chars.next().unwrap().to_digit(10).unwrap() as i32;
                }
                Some('*') => {
                    val *= chars.next().unwrap().to_digit(10).unwrap() as i32;
                }
                Some('/') => {
                    let next_val = chars.next().unwrap().to_digit(10).unwrap() as i32;
                    if next_val == 0 || val % next_val != 0 {
                        valid = false;
                        break; // Invalid operation, skip
                    }
                    val /= next_val;
                }
                Some(c) => {
                    // Continue processing digits
                    panic!("Unexpected character in expression: {}", c);
                }
                None => break, // End of string
            };
        }

        if valid && val == 10 {
            results.push(sol.clone());
        }
    }
    results
}

pub fn runner() {
    let combinations = prepare("98765");

    // no threads
    let t0 = std::time::Instant::now();
    let verified = verify(&combinations);
    println!("[no thread] found {} solutions in {:?}", verified.len(), t0.elapsed());

    for nthread in 2..16 {
        thread::scope(|s| {
            let size = combinations.len() / nthread;
            let mut results = vec![];
            let mut jhandles = vec![];
            let t0 = std::time::Instant::now();
            for i in 0..nthread {
                let start = i * size;
                let end = if i == nthread - 1 { combinations.len() } else { start + size };
                let chunk = &combinations[start..end];

                jhandles.push(s.spawn(move || {
                    verify(chunk)
                }));
            }

            for handle in jhandles {
                results.extend(handle.join().unwrap());
            }

            println!("[{} threads] found {} solutions in {:?}", nthread, results.len(), t0.elapsed());
        })
    }
}


#[test]
fn test_mk_ops() {
    runner();
}
