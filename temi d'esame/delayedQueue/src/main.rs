mod delayed_queue {
    use std::cmp::Ordering;
    use std::collections::BinaryHeap;
    use std::sync::Mutex;
    use std::thread;
    use std::time::Instant;

    pub struct Item<T> {
        i: Instant,
        t: T,
    }

    pub struct DealyedQueue<T> {
        queue: Mutex<BinaryHeap<Item<T>>>,
    }

    impl<T: Send> Eq for Item<T> {}

    impl<T: Send> PartialEq<Self> for Item<T> {
        fn eq(&self, other: &Self) -> bool {
            self.i.eq(&other.i)
        }
    }

    impl<T: Send> PartialOrd<Self> for Item<T> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            other.i.partial_cmp(&other.i)
        }
    }

    impl<T: Send> Ord for Item<T> {
        fn cmp(&self, other: &Self) -> Ordering {
            other.i.cmp(&other.i)
        }
    }

    impl<T: Ord> DealyedQueue<T> {
        pub fn new() -> Self {
            DealyedQueue {
                queue: Mutex::new(BinaryHeap::new()),
            }
        }

        pub fn offer(&self, t: T, i: Instant) {
            let mut queue = self.queue.lock().expect("Mutex poisoned");
            queue.push((i, t));
        }

        pub fn take(&self) -> Option<T> {
            let queue = self.queue.lock().expect("Mutex poisoned");
            let a = queue.peek();
            if a.is_some() {
                let b = a.unwrap();
                let c = Instant::now();
                if (*b).0.lt(&c) {
                    let d = c - b.0;
                    drop(queue);
                    thread::sleep(d);
                }
                let mut queue = self.queue.lock().expect("Mutex poisoned");
                let d = queue.pop().unwrap();
                Some(d.1)
            } else {
                None
            }
        }

        pub fn size(&self) -> usize {
            let a = self.queue.lock().expect("Mutex poisoned");
            a.len()
        }
    }
}

fn main() {
    println!("Hello, world!");
}
