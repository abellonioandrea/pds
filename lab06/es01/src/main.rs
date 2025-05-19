use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("First");
    for i in 1..17 {
        let t1 = std::time::Instant::now();
        find_primes1(1000000, i);
        let t2 = std::time::Instant::now();
        println!("{:?} threads, time:{:?}", i, t2 - t1);
    }

    println!("Second");
    for i in 1..17 {
        let t1 = std::time::Instant::now();
        let res = find_primes2(1000000, i);
        let t2 = std::time::Instant::now();
        println!("{:?} threads, time:{:?}", i, t2 - t1);
    }
}

pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u64) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn find_primes1(limit: u64, n_threads: u64) -> Vec<u64> {
    let counter_shared = Arc::new(Mutex::new(2));
    let res_shared = Arc::new(Mutex::new(Vec::new()));
    let mut threads = vec![];

    for _ in 0..n_threads {
        let counter = counter_shared.clone();
        let res = res_shared.clone();
        threads.push(thread::spawn(move || {
            loop {
                let num = {
                    let mut counter = counter.lock().unwrap();
                    if *counter > limit {
                        break;
                    }
                    let num = *counter;
                    *counter += 1;
                    num
                };

                if is_prime(num) {
                    res.lock().unwrap().push(num);
                }
            }
        }));
    }

    for t in threads {
        t.join().unwrap();
    }

    let result = res_shared.lock().unwrap();
    result.clone()
}

pub fn find_primes2(limit: u64, n_threads: u64) -> Vec<u64> {
    let mut threads = vec![];
    for i in 1..n_threads + 1 {
        threads.push(thread::spawn(move || {
            let mut vet: Vec<u64> = vec![];
            for x in ((1 + i)..limit).step_by(n_threads as usize) {
                if is_prime(x) {
                    vet.push(x);
                }
            }
            vet
        }));
    }
    let mut result: Vec<u64> = vec![];
    for t in threads {
        result.append(&mut t.join().unwrap());
    }
    result
}
