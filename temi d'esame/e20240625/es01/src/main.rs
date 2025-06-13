use std::sync::mpsc::{Receiver, Sender, channel};
use std::time::Duration;

struct Exchanger<T: Send> {
    tx: Sender<T>,
    rx: Receiver<T>,
}

impl<T: Send> Exchanger<T> {
    fn new() -> (Self, Self) {
        let (tx1, rx1) = channel();
        let (tx2, rx2) = channel();

        (Exchanger { tx: tx1, rx: rx2 }, Exchanger { tx: tx2, rx: rx1 })
    }

    fn exchange(&self, t: T) -> Option<T> {
        self.tx.send(t).expect("TODO: panic message");
        match self.rx.recv() {
            Ok(a) => {
                Some(a)
            }
            Err(a) => {
                None
            }
        }
    }
}

fn main() {
    let (ch1, ch2): (Exchanger<usize>, Exchanger<usize>) = Exchanger::new();
    let th1 = std::thread::spawn(move || {
        println!("{}", ch1.exchange(1).unwrap());
    });
    let th2 = std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(2000));
        println!("{}", ch2.exchange(2).unwrap());
    });
    th1.join();
    th2.join();
}
