use std::ops::{Add, AddAssign, Deref, SubAssign};
use std::sync::{Arc, Condvar, Mutex};

pub struct CyclicBarrier {
    n: usize,
    count: Mutex<usize>,
    external: Mutex<bool>,
    internal: Mutex<bool>,
    cv_internal: Condvar,
    cv_external: Condvar,
}

impl CyclicBarrier {
    fn new(n: usize) -> Self {
        CyclicBarrier {
            n,
            count: Mutex::from(0),
            external: Mutex::from(true),
            internal: Mutex::from(false),
            cv_internal: Condvar::new(),
            cv_external: Condvar::new(),
        }
    }

    fn wait(&self) {
        {
            //println!("waiting for external lock");
            let external = self.external.lock().unwrap();
            //println!("after external lock");
            if external.eq(&false) { //porta esterna chiusa, aspetto
                //println!("waiting on external");
                self.cv_external.wait_while(external, |a| { *a == false }).expect("Boh");
                //println!("after waiting on external");
            }
        }
        {
            //println!("waiting for count lock");
            let mut count = self.count.lock().expect("Mutex poisoned");
            //println!("after count lock");
            count.add_assign(1);
            if (count.eq(&self.n)) {
                drop(count);
                self.cv_internal.notify_all();
            } else {
                drop(count);
                let internal = self.internal.lock().unwrap();
                //println!("waiting for all");
                self.cv_internal.wait_while(internal, |a| { *a == false }).expect("Mutex poisoned"); //aspetto sulla interna
                //println!("after waiting for all");
            }
        }
        //println!("waiting for external lock 2");
        let mut external = self.external.lock().unwrap();
        //println!("after external lock 2");
        *external = false;
        drop(external);
        let mut internal = self.internal.lock().unwrap();
        //println!("after internal lock 2");
        *internal = true;
        drop(internal);
        //println!("waiting for count lock 2");
        let mut count = self.count.lock().expect("Mutex poisoned");
        // println!("after count lock 2");
        count.sub_assign(1);
        self.cv_internal.notify_all();
        if count.eq(&0) {
            let mut external = self.external.lock().unwrap();
            *external = true;
            // println!("set external true");
            drop(external);
            let mut internal = self.internal.lock().unwrap();
            // println!("after internal lock 2");
            *internal = false;
            drop(internal);
            self.cv_external.notify_all();
        }
    }
}

fn main() {
    let abarrrier = Arc::new(CyclicBarrier::new(3));
    let mut vt = Vec::new();
    for i in 0..3 {
        let cbarrier = abarrrier.clone();
        vt.push(std::thread::spawn(move || {
            for j in 0..10 {
                cbarrier.wait();
                println!("after barrier {} {}", i, j);
            }
        }));
    }
    for t in vt {
        t.join().unwrap();
    }
}
