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
    cv_p: Arc<(Mutex<bool>, Condvar)>, //condition variable per produttore
    cv_c: Arc<(Mutex<bool>, Condvar)>, //condition variable per consumatore
    close: Mutex<bool>,
}
impl<T> MyChannel<T> {
    pub fn new(n: usize) -> Self {
        MyChannel {
            queue: Mutex::new(VecDeque::with_capacity(n)),
            cv_p: Arc::new((Mutex::new(false), Condvar::new())),
            cv_c: Arc::new((Mutex::new(false), Condvar::new())),
            close: Mutex::new(false),
        }
    }

    pub fn write(&self, item: Item<T>) -> Result<(), ()> {
        if self.close.lock().unwrap().eq(&false) {
            let mut producer_lock = self.cv_p.0.lock().unwrap();
            let mut queue_lock = self.queue.lock().unwrap();
            while queue_lock.len() == queue_lock.capacity() {
                producer_lock = self.cv_p.1.wait(producer_lock).unwrap(); //metto in attesa i produttori finchè il consumatore non avrà consumato almeno un elemento
            }
            queue_lock.push_back(item);

            self.cv_c.1.notify_one(); //sblocco eventuali consumatori in attesa
            return Ok(());
        }
        Err(())
    }

    pub fn read(&self) -> Result<Item<T>, ()> {
        let mut consumer_lock = self.cv_c.0.lock().unwrap();
        let mut queue_lock = self.queue.lock().unwrap();
        while queue_lock.len() == 0 {
            if self.close.lock().unwrap().eq(&true) {
                return Err(());
            }
            //self.cv_c.1.wait(self.cv_c.0.lock().unwrap());
            consumer_lock = self.cv_c.1.wait(consumer_lock).unwrap();  //blocco i consumatori finchè non c'è un altro elemento prodotto
        }

        self.cv_p.1.notify_one(); //sblocco tutti i produttori
        Ok(queue_lock.pop_front().unwrap())
    }

    pub fn close(&self) {
        let mut aa = self.close.lock().unwrap();
        *aa = true;
        self.cv_c.1.notify_all();
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
