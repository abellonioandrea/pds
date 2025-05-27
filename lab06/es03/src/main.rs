use crate::Item::{Stop, Value};
use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Condvar, Mutex};

#[derive(Debug)]
pub enum Item<T> {
    Value(T),
    Stop,
}

pub struct MyChannel<T> {
    queue: Mutex<VecDeque<Item<T>>>,
    cv: Condvar,
    close: Mutex<bool>,
}
impl<T> MyChannel<T> {
    pub fn new(n: usize) -> Self {
        MyChannel {
            queue: Mutex::new(VecDeque::with_capacity(n)),
            cv: Condvar::new(),
            close: Mutex::new(false),
        }
    }

    pub fn write(&self, item: Item<T>) -> Result<(), ()> {
        let mut data = self.queue.lock().expect("Mutex poisoned");
        while (data.len() == data.capacity()) {
            data = self.cv.wait(data).expect("Mutex poisoned");
        }
        data.push_back(item);
        drop(data);
        self.cv.notify_one();
        Ok(())
    }

    pub fn read(&self) -> Result<Item<T>, ()> {
        let mut data = self.queue.lock().expect("Mutex poisoned");
        while (data.len() == 0) {
            data = self.cv.wait(data).expect("Mutex poisoned");
        }
        let a = data.pop_front();
        drop(data);
        self.cv.notify_one();
        Ok(a.unwrap())
    }

    pub fn close(&self) {
        let mut aa = self.close.lock().unwrap();
        *aa = true;
        self.cv.notify_all();
    }
}

fn produttore(ch: Arc<MyChannel<usize>>, n: usize) {
    for elem in 0..n {
        {
            ch.write(Value(elem)).unwrap();
            println!("Producer: {:?}", elem);
        }
    }
    {
        let val = ch.write(Stop).unwrap();
        ch.close();
        val
    }
}

fn consumatore(ch: Arc<MyChannel<usize>>) {
    loop {
        println!("1");
        let item = {
            ch.read()
        };
        match item {
            Ok(Item::Value(val)) => println!("Consumer: {:?}", val),
            Ok(Item::Stop) | Err(_) => break,
        }
    }
}

fn main() {
    let ch = MyChannel::new(10);
    let ch_arc = Arc::new(ch);

    let ch_clone_prod = Arc::clone(&ch_arc);
    let ch_clone_cons = Arc::clone(&ch_arc);

    let prod_handle = std::thread::spawn(move || {
        produttore(ch_clone_prod, 10);
    });

    let cons_handle = std::thread::spawn(move || {
        consumatore(ch_clone_cons);
    });

    prod_handle.join().unwrap();
    cons_handle.join().unwrap();
}
