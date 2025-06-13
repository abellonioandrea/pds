use crate::exchanger::Exchanger;

mod exchanger {
    use std::sync::mpsc::{channel, Receiver, Sender};

    pub struct Exchanger<T: Send> {
        rx: Receiver<T>,
        tx: Sender<T>,
    }

    impl<T: Send> Exchanger<T> {
        pub fn new() -> Self {
            let (tx, rx) = channel();
            Exchanger {
                rx,
                tx,
            }
        }

        pub fn exchange(&self, t: T) -> Option<T> {
            self.tx.send(t).expect("Error");
            match self.rx.recv() {
                Ok(t) => {
                    Some(t)
                }
                Err(e) => {
                    None
                }
            }
        }
    }
}

fn main() {
    let a = Exchanger::<u8>::new();
    let b = Exchanger::<u8>::new();
    println!("{:?}", a.exchange(1));
    println!("{:?}", b.exchange(2));
}
