use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

enum Datum<T> {
    Value(T),
    Stop,
}

pub struct MyChannel<T> {
    queue: Mutex<VecDeque<Datum<T>>>,
    cv: Condvar,
}

impl<T> MyChannel<T> {
    pub fn new(n: usize) -> Self {
        MyChannel {
            queue: Mutex::new(VecDeque::with_capacity(n)),
            cv: Condvar::new(),
        }
    }

    pub fn write(&self, value: T) -> Result<(), ()> {
        let mut queue = self.queue.lock().unwrap();

        if let Some(Datum::Stop) = queue.back() {
            return Err(());
        }

        while queue.len() == queue.capacity() {
            println!("Queue is full, waiting to write...");
            queue = self.cv.wait(queue).unwrap();
        }

        queue.push_back(Datum::Value(value));
        self.cv.notify_one();
        Ok(())
    }

    pub fn read(&self) -> Result<T, ()> {
        let mut queue = self.queue.lock().unwrap();

        if let Some(Datum::Stop) = queue.front() {
            return Err(());
        }

        while queue.is_empty() {
            println!("Queue is empty, waiting to read...");
            queue = self.cv.wait(queue).unwrap();
        }

        let ret = match queue.pop_front().unwrap() {
            Datum::Value(value) => Ok(value),
            _ => panic!("Unexpected data type"),
        };
        self.cv.notify_one();
        ret
    }

    pub fn stop(&self) {
        let mut queue = self.queue.lock().unwrap();

        if let Some(Datum::Stop) = queue.back() {
            return;
        }

        queue.push_back(Datum::Stop);
        self.cv.notify_all();
    }
}

fn example_usage() {
    let channel = Arc::new(MyChannel::new(10));

    // Writer thread
    let writer = std::thread::spawn({
        let channel = channel.clone();
        move || {
            for i in 0..20 {
                if let Err(_) = channel.write(i) {
                    break; // the reader has closed the channel
                }
                // Simulate some delay in producing data
                // thread::sleep(Duration::from_millis(100));
            }
            channel.stop();
        }
    });

    // Reader thread
    let reader = std::thread::spawn({
        let channel = channel.clone();
        move || {
            loop {
                match channel.read() {
                    Ok(value) => println!("Read value: {}", value),
                    Err(_) => break, // channle has been stopped
                }
                // Simulate some processing time
                // thread::sleep(Duration::from_millis(100));
            }
        }
    });

    writer.join().unwrap();
    reader.join().unwrap();
}

#[test]
fn test_my_channel() {
    example_usage();
}