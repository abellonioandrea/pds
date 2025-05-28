use std::{thread, time::Duration, vec};


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

pub fn find_primes(limit: u64, n_threads: u64) -> Vec<u64> {
    let mut results = vec![];
    let mut primes = vec![];

    let t0 = std::time::Instant::now();

    for i in 0..n_threads {
        let jh = thread::spawn(move || {
            let mut primes = vec![];

            let mut j = 2 + i;

            while j < limit {
                // print("Thread")
                if is_prime(j) {
                    primes.push(j);
                }
                j += n_threads;
            }
            primes
        });
        results.push(jh);
    }

    for jh in results {
        match jh.join() {
            Ok(res) => {
                primes.extend_from_slice(&res[..]);
            }
            Err(_) => {
                println!("Thread panicked");
            }
        };
    }

    // println!("{:?}", primes);
    println!("[{} threads - S] Time taken: {:?} {} ", n_threads, t0.elapsed(), primes.len());

    primes
}

pub fn find_primes_with_counter(limit: u64, n_threads: u64) -> Vec<u64> {
    let mut results = vec![];
    let mut primes = vec![];

    let t0 = std::time::Instant::now();

    let counter = std::sync::Arc::new(std::sync::Mutex::new(0));

    for i in 0..n_threads {
        let counter = counter.clone();
        let jh = thread::spawn(move || {
            let mut primes = vec![];
            loop {
                let mut j = counter.lock().unwrap();
                if *j >= limit {
                    break;
                }
                let n = *j;
                *j += 1;
                drop(j);

                if is_prime(n) {
                    primes.push(n);
                }
            }
            primes
        });
        results.push(jh);
    }

    for jh in results {
        match jh.join() {
            Ok(res) => {
                primes.extend_from_slice(&res[..]);
            }
            Err(_) => {
                println!("Thread panicked");
            }
        };
    }

    println!("[{} threads - C] Time taken: {:?} {} ", n_threads, t0.elapsed(), primes.len());

    primes
}

#[test]
fn test_found_primes() {
    for n_threads in 1..=8 {
        let primes = find_primes(1_000_000, n_threads);
        //let primes = find_primes_with_counter(1_000_000, n_threads);
    }
}
