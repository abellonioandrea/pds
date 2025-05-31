use es02::CyclicBarrier;

mod es02 {
    use std::sync::Arc;
    use std::sync::mpsc::{channel, Receiver, Sender};

    pub struct CyclicBarrier {
        n: usize,
        senders: Vec<Arc<Sender<()>>>,
        receivers: Vec<Receiver<()>>,
        counter: usize,
    }

    pub struct Waiter {
        tx: Vec<Arc<Sender<()>>>,
        rx: Receiver<()>,
    }

    impl Waiter {
        fn new(tx: Vec<Arc<Sender<()>>>, rx: Receiver<()>) -> Self {
            Waiter { tx, rx }
        }

        pub fn wait(&self) {
            for x in 0..self.tx.len() {
                self.tx[x].send(()).expect("TODO: panic message"); //invio n segnali (anche se stesso)
            }

            for _ in 0..self.tx.len() {
                self.rx.recv().expect("TODO: panic message"); //aspetto n segnali
            }
        }
    }

    impl CyclicBarrier {
        pub fn new(n: usize) -> Self {
            let mut senders = Vec::new();
            let mut receivers = Vec::new();
            for _ in 0..n {
                let (tx, rx) = channel();
                senders.push(Arc::from(tx));
                receivers.push(rx);
            }

            CyclicBarrier {
                n,
                senders,
                receivers,
                counter: 0,
            }
        }

        pub fn get_waiter(&mut self) -> Waiter {
            let rx = self.receivers.remove(0);
            self.counter = (self.counter + 1) % self.n; //riparto da zero
            Waiter::new(self.senders.clone(), rx)
        }
    }
}

fn main() {
    let mut cbarrrier = CyclicBarrier::new(3);
    let mut vt = Vec::new();
    for i in 0..3 {
        let waiter = cbarrrier.get_waiter();
        vt.push(std::thread::spawn(move || {
            for j in 0..10 {
                waiter.wait();
                println!("after barrier, thread: {}, cycle: {}", i, j);
            }
        }));
    }
    for t in vt {
        t.join().unwrap();
    }
}
