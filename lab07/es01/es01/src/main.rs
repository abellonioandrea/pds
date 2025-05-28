use std::{sync::Arc, thread, time::Duration};

use es01::CountDownLatch;

mod es01 {
    use std::{
        ops::SubAssign,
        sync::{Condvar, Mutex},
    };

    pub struct CountDownLatch {
        n: Mutex<usize>,
        cv: Condvar,
    }

    impl CountDownLatch {
        pub fn new(n: usize) -> Self {
            CountDownLatch {
                n: Mutex::new(n),
                cv: Condvar::new(),
            }
        }

        // wait zero aspetta al massimo timeout ms

        // se esce per timeout ritorna Err altrimenti Ok

        pub fn wait_zero(&self, timeout: Option<std::time::Duration>) -> Result<(), ()> {
            let mut data = self.n.lock().expect("Mutex poisoned");
            if timeout.is_none() {
                data = self.cv.wait_while(data, |c| *c > 0).unwrap();
                Ok(())
            } else {
                let res = self
                    .cv
                    .wait_timeout_while(data, timeout.unwrap(), |c| *c > 0)
                    .expect("Mutex poisoned");
                if res.1.timed_out() { Err(()) } else { Ok(()) }
            }
        }

        pub fn count_down(&self) {
            let mut data = self.n.lock().expect("Mutex posisoned");
            data.sub_assign(1);
            if data.eq(&0) {
                drop(data);
                self.cv.notify_all();
            }
        }
    }
}

fn main() {
    let latch = Arc::new(CountDownLatch::new(10));
    let latch2 = Arc::new(CountDownLatch::new(1));
    for _ in 0..10 {
        let latch2copy = Arc::clone(&latch2);
        let latchcopy = Arc::clone(&latch);
        thread::spawn(move || {
            latch2copy.wait_zero(Some(Duration::from_millis(100))).unwrap();
            do_some_work("(2) lavoro che necessita driver");
            latchcopy.count_down();
            do_some_work("(3) altro lavoro che non necessita driver");
        });
    }
    do_some_work("(1) prepara il driver");
    latch2.count_down();
    latch.wait_zero(Some(Duration::from_millis(100))).unwrap();
    do_some_work("(4) rilascia il driver");
}

fn do_some_work(testo: &str) {
    thread::sleep(Duration::from_millis(10));
    println!("{}", testo);
}
