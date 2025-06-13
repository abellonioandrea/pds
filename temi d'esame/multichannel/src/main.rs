mod multi_channel {
    use std::sync::mpsc::{channel, Receiver, SendError, Sender};
    use std::sync::Mutex;

    struct MultiChannel {
        channels: Mutex<Vec<Sender<u8>>>,
    }

    impl MultiChannel {
        fn new() -> Self {
            MultiChannel {
                channels: Mutex::new(Vec::new())
            }
        }

        fn subscribe(&self) -> Receiver<u8> {
            let mut lock = self.channels.lock().unwrap();
            let (tx, rx) = channel();
            lock.push(tx);
            rx
        }

        fn send(&self, data: u8) -> Result<(), SendError<u8>> {
            let lock = self.channels.lock().unwrap();
            for e in lock.iter() {
                match e.send(data) {
                    Ok(..) => {}
                    Err(e) => { return Err(e) }
                }
            }
            Ok(())
        }
    }
}

fn main() {
    println!("Hello, world!");
}
