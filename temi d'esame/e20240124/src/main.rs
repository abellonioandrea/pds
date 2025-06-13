use std::result::Result;
use std::sync::{Mutex};
use std::sync::mpsc::{channel, Receiver, SendError, Sender};

struct MultiChannel {
    txs: Mutex<Vec<Sender<u8>>>,
}

impl MultiChannel {
    fn new() -> Self {
        MultiChannel {
            txs: Mutex::new(Vec::new())
        }
    }

    fn subscribe(&self) -> Receiver<u8> {
        let (tx, rx) = channel();
        let mut lock = self.txs.lock().unwrap();
        lock.push(tx);
        rx
    }

    fn send(&self, data: u8) -> Result<(), SendError<u8>> {
        let ths = self.txs.lock().unwrap();
        if ths.len() == 0 {
            return Err(SendError(data));
        }
        for th in ths.iter() {
            th.send(data)?;
        }
        Ok(())
    }
}

fn main() {}
