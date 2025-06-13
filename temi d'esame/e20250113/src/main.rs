use std::sync::Mutex;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

struct DelayedExecutor {
    threads: Mutex<Vec<JoinHandle<()>>>,
    open: Mutex<bool>,
}

impl DelayedExecutor {
    fn new() -> Self {
        DelayedExecutor {
            threads: Mutex::new(Vec::new()),
            open: Mutex::new(true),
        }
    }

    fn execute<F: FnOnce() + Send + 'static>(&self, f: F, delay: Duration) -> bool {
        let open = self.open.lock().unwrap();
        if *open == true {
            let mut ths = self.threads.lock().unwrap();
            ths.push(std::thread::spawn(move || {
                let start = Instant::now();
                while (Instant::now() - start).lt(&delay) {}
                f();
            }));
            true
        } else {
            false
        }
    }

    fn close(&self, drop_pending_tasks: bool) {
        if !drop_pending_tasks {
            let mut ths = self.threads.lock().unwrap();
            while ths.len() != 0 {
                let th = ths.pop();
                th.unwrap().join().expect("errore");
            }
        }
    }
}

fn main() {
    let de = DelayedExecutor::new();
    de.execute(|| { println!("ciao") }, Duration::from_secs(1));
    de.execute(|| { println!("prima") }, Duration::from_millis(100));
    de.execute(|| { println!("dopo") }, Duration::from_secs(2));
    de.close(false);
}
