use std::sync::{Mutex, Condvar};

struct BarrierState {
    waiting: usize,
    exiting: bool,
}

pub struct CiclicBarrier {
    size: usize,
    state: Mutex<BarrierState>,
    cond: Condvar,

}


impl CiclicBarrier {
    pub fn new(n: usize) -> CiclicBarrier {
        CiclicBarrier {
            size: n,
            state: Mutex::new(BarrierState { waiting: 0, exiting: false }),
            cond: Condvar::new(),
        }
    }

    pub fn wait(&self) {
        let mut state = self.state.lock().unwrap();

        // can't go on if the barrier is open (exiting)
        state = self.cond.wait_while(state, |state| state.exiting).unwrap();

        state.waiting += 1;

        if state.waiting == self.size {
            state.exiting = true;
            self.cond.notify_all();
        } else {
            state = self.cond.wait_while(state, |state| !state.exiting).unwrap();
        }

        state.waiting -= 1;
        if state.waiting == 0 {
            state.exiting = false;
            self.cond.notify_all();
        }
    }
}

