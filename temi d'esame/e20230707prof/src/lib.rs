mod delayed_queue {
    use std::cmp::Ordering;
    use std::collections::BinaryHeap;
    use std::sync::{Condvar, Mutex};
    use std::time::Instant;

    //item deve essere odinabile
    //send per threadsafe
    struct Item<T: Send> {
        t: T,
        i: Instant,
    }

    impl<T: Send> PartialEq for Item<T> {
        fn eq(&self, other: &Self) -> bool {
            self.i.eq(&other.i)
        }
    }

    impl<T: Send> Eq for Item<T> {} //non serve implementarla, già fatto sopra

    impl<T: Send> PartialOrd for Item<T> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            other.i.partial_cmp(&self.i) //il più piccolo davanti (binaryheap mette il più grande prima)
        }
    }

    impl<T: Send> Ord for Item<T> {
        fn cmp(&self, other: &Self) -> Ordering {
            other.i.cmp(&self.i)
        }
    }

    pub struct DelayedQueue<T: Send> {
        //mutex per threadsafe
        //mettere una tupla nel binary heap non va bene perchè è difficile da ordinare
        data: Mutex<BinaryHeap<Item<T>>>,
        cv: Condvar,
    }

    impl<T: Send> DelayedQueue<T> {
        pub fn new() -> DelayedQueue<T> {
            Self {
                data: Mutex::new(BinaryHeap::new()),
                cv: Condvar::new(),
            }
        }

        pub fn offer(&self, t: T, i: Instant) {
            let mut data = self.data.lock().expect("Mutex poisoned");
            data.push(Item { t, i });
            drop(data); //droppo prima della notify per permettere di svegliarsi subito
            self.cv.notify_all();
        }

        pub fn take(&self) -> Option<T> {
            let mut data = self.data.lock().expect("Mutex poisoned");
            loop {
                let now = Instant::now();
                let first = data.peek();
                if let Some(item) = first {
                    println!("Cheking item expiring on {:?}", item.i);
                    let i = item.i;
                    if i < now {
                        let res = data.pop().unwrap();
                        return Some(res.t);
                    } else {
                        let d = i.duration_since(now);
                        println!("Sleeping for {:?}", d);
                        data = self
                            .cv
                            .wait_timeout(data, d)
                            .expect("Mutex poisoned")
                            .0;
                    }
                } else {
                    return None;
                }
            }
        }

        pub fn size(&self) -> usize {
            let data = self.data.lock().expect("Mutex poisoned");
            data.len()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::delayed_queue::DelayedQueue;
    use std::ops::Add;
    use std::time::{Duration, Instant};

    #[test]
    fn an_empty_queue_returns_none() {
        let q = DelayedQueue::<i32>::new();
        assert_eq!(q.take(), None);
        assert_eq!(q.size(), 0);
    }

    #[test]
    fn items_are_returned_in_order() {
        let q = DelayedQueue::new();
        let now = Instant::now();
        q.offer(1500, now.add(Duration::from_millis(10)));
        q.offer(500, now.add(Duration::from_millis(5)));
        assert_eq!(q.take(), Some(500));
        assert_eq!(q.take(), Some(1500));
        assert_eq!(q.take(), None);
    }

    #[test]
    fn items_are_returned_in_order_even_if_inserted_after_waiting_starts() {
        let q = DelayedQueue::new();
        std::thread::scope(|s| {
            let now = Instant::now();
            q.offer(42, now.add(Duration::from_millis(10)));
            s.spawn(|| {
                assert_eq!(q.take(), Some(20));
                assert_eq!(q.take(), Some(42));
                assert_eq!(q.take(), None);
            });
            s.spawn(|| {
                std::thread::sleep(Duration::from_millis(2));
                q.offer(20, Instant::now().add(Duration::from_millis(1)));
            });
        });
    }

    #[test]
    fn items_are_returned_in_order_even_if_inserted_after_waiting_starts2() {
        let q = DelayedQueue::new();
        std::thread::scope(|s| {
            let now = Instant::now();
            q.offer(42, now.add(Duration::from_millis(10)));
            s.spawn(|| {
                assert_eq!(q.take(), Some(42));
                assert_eq!(q.take(), Some(20));
                assert_eq!(q.take(), None);
            });
            s.spawn(|| {
                std::thread::sleep(Duration::from_millis(2));
                q.offer(20, Instant::now().add(Duration::from_millis(10)));
            });
        });
    }

    #[test]
    fn method_size_works() {
        let q = DelayedQueue::new();
        q.offer(1500, Instant::now());
        assert_eq!(q.size(), 1);
    }

    #[test]
    fn two_threads_reading_the_queue_work() {
        let q = DelayedQueue::new();
        q.offer(1500, Instant::now() + Duration::from_millis(10));
        q.offer(500, Instant::now() + Duration::from_millis(5));
        std::thread::scope(|s| {
            for _ in 0..2 {
                s.spawn(|| {
                    let r = q.take();
                    assert!(r == Some(500) || r == Some(1500));
                });
            }
        });
    }
}
