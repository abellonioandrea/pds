mod count_down_lock {
    use std::sync::{Condvar, Mutex};
    use std::time::Duration;

    pub struct CountDownLock {
        counter: Mutex<usize>,
        cv: Condvar,
    }

    impl CountDownLock {
        pub fn new(n: usize) -> Self {
            CountDownLock {
                counter: Mutex::new(n),
                cv: Condvar::new(),
            }
        }

        pub fn count_down(&self) {
            let mut counter = self.counter.lock().expect("Mutex poisoned");
            *counter -= 1;
            if *counter == 0 {
                drop(counter);
                self.cv.notify_all();
            }
        }

        pub fn wait(&self) {
            let counter = self.counter.lock().expect("Mutex poisoned");
            self.cv.wait_while(counter, |c| { *c > 0 }).expect("TODO: panic message");
        }

        pub fn wait_timeout(&self, d: Duration) -> std::sync::WaitTimeoutResult {
            let counter = self.counter.lock().expect("Mutex poisoned");
            self.cv.wait_timeout_while(counter, d, |c| { *c > 0 }).unwrap().1
        }
    }
}

fn main() {
    println!("Hello, world!");
}
