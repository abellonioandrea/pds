use itertools::{Itertools};
use std::sync::Arc;
use std::thread;

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

#[test]
fn test_mk_ops() {
    let symbols = vec!['+', '-', '*', '/'];
    let n = 4;
    let result = mk_ops(&symbols, n);
    assert_eq!(result.len(), symbols.len().pow(n as u32));

    let res = prepare("23423");
    println!("{} {:?}", res.len(), res.iter().take(n).collect::<Vec<_>>());

    verify(&*res);
}

pub fn verify(v: &[String]) -> Vec<String> {
    let thread_num = 16;
    let result: Vec<String> = vec![];
    let dim = v.len() / thread_num;
    let mut threads = vec![];
    let v = Arc::new(v.to_vec());

    for i in 0..thread_num {
        let v1 = Arc::clone(&v);
        threads.push(thread::spawn(move || {
            let data = &v1[dim * i..dim * (i + 1)];
            let mut res_vec: Vec<String> = vec![];
            for row in data {
                let mut val = row.chars();
                let mut res: isize = val.nth(0).unwrap() as isize;
                res -= 48;
                for _ in 0..4 {
                    let mut op = val.nth(0).unwrap();
                    let mut next_op = val.nth(0).unwrap() as isize;
                    next_op -= 48;
                    match op {
                        '+' => {
                            res += next_op;
                        }
                        '-' => {
                            res -= next_op;
                        }
                        '*' => {
                            res *= next_op;
                        }
                        '/' => {
                            if next_op != 0 && res % next_op == 0 { //solo se la divisione non è per zero e non è frazionaria
                                res /= next_op;
                            }
                        }
                        _ => {
                            res = res;
                        }
                    }
                }
                if res == 10 {
                    res_vec.push(row.clone());
                }
            }
            res_vec
        }));
    }

    for thread in threads {
        println!("{:?}", thread.join().unwrap());
    }

    result
}

fn main() {
    println!("Hello, world!");
}
