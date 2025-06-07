use std::{sync::{Arc, Condvar, Mutex}, thread, time::Duration};
pub struct CountDownLatch {
    count: Mutex<usize>,
    cv: Condvar,
}

impl CountDownLatch {
    pub fn new(count: usize) -> Self {
        CountDownLatch {
            count: Mutex::new(count),
            cv: Condvar::new(),
        }
    }

    pub fn wait_zero(&self, timeout: Option<std::time::Duration>) -> Result<(), ()> {
        let mut count = self.count.lock().unwrap();
        while *count > 0 {
            if let Some(duration) = timeout {
                let to_result;
                (count, to_result) = self.cv.wait_timeout(count, duration).unwrap();
                if to_result.timed_out() {
                    return Err(());
                }
            } else {
                count = self.cv.wait(count).unwrap();
            }
        }
        Ok(())
    }

    pub fn count_down(&self) {
        let mut count = self.count.lock().unwrap();
        *count -= 1;
        self.cv.notify_all();
    }
}

pub fn do_some_work(description: &str) {
    thread::sleep(Duration::from_millis(1000));
    println!("working: {}\n", description);
    thread::sleep(Duration::from_millis(1000));
}

pub fn demo() {
    let driver_ready = Arc::new(CountDownLatch::new(1));
    let driver_release = Arc::new(CountDownLatch::new(10));
    let mut handles = vec![];

    for i in 0..10 {
        let driver_ready = Arc::clone(&driver_ready);
        let driver_release = Arc::clone(&driver_release);
        let h = thread::spawn(move || {
            driver_ready.wait_zero(None).unwrap();
            do_some_work("(2) lavoro che necessita driver");
            driver_release.count_down();
            do_some_work("(3) altro lavoro che non necessita driver");
            println!("thread finished {}", i);
        });
        handles.push(h);
    }

    do_some_work("(1) prepapara il driver");
    driver_ready.count_down();
    driver_release.wait_zero(None).unwrap();
    do_some_work("(4) rilascia il driver");

    for h in handles {
        let _ = h.join();
    }
}

#[test]
fn test_count_down_latch() {
    demo();
}