use itertools::Itertools;
use std::sync::mpsc;

pub struct CyclicBarrier {
    senders: Vec<mpsc::Sender<()>>,
    receivers: Vec<mpsc::Receiver<()>>,
}

impl CyclicBarrier {
    pub fn new(n: usize) -> Self {
        let mut senders = Vec::new();
        let mut receivers = Vec::new();
        for _ in 0..n {
            let (tx, rx) = mpsc::channel();
            senders.push(tx);
            receivers.push(rx);
        }
        CyclicBarrier { senders, receivers }
    }

    pub fn get_waiter(&mut self) -> Result<Waiter, &'static str> {
        if self.receivers.len() == 0 {
            Err("No more waiters available")
        } else {
            let idx = self.senders.len() - self.receivers.len();
            let receiver = self.receivers.pop().unwrap();
            let senders = self
                .senders
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != idx)
                .map(|(_, s)| s.clone())
                .collect_vec();
            Ok(Waiter {
                receiver,
                senders: senders,
            })
        }
    }
}

pub struct Waiter {
    receiver: mpsc::Receiver<()>,
    senders: Vec<mpsc::Sender<()>>,
}

impl Waiter {
    pub fn wait(&self) {
        self.senders.iter().for_each(|s| {
            s.send(()).unwrap();
        });

        for _ in 0..self.senders.len() {
            self.receiver.recv().unwrap();
        }
    }
}
