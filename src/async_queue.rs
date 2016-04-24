use std::sync::{Condvar, Mutex};
use std::collections::VecDeque;

pub struct AsyncQueue<T>{
    pub queue: Mutex<VecDeque<T>>,
    condvar: Condvar,
}

impl<T> AsyncQueue<T>{
    pub fn new() -> AsyncQueue<T>{
        AsyncQueue::<T>{
            queue: Mutex::new(VecDeque::new()),
            condvar: Condvar::new()
        }
    }

    pub fn try_deq(&self) -> Option<T>{
        let mut q = unwrap_result!(self.queue.lock());
        q.pop_front()
    }

    pub fn deq(&self) -> T{
        let mut q = unwrap_result!(self.queue.lock());
        while let None = q.front(){
            q = unwrap_result!(self.condvar.wait(q));
        }
        q.pop_front().unwrap()
    }

    pub fn enq(&self, item: T){
        let mut q = unwrap_result!(self.queue.lock());
        q.push_back(item);
        self.condvar.notify_one();
    }
}

#[cfg(test)]
mod test{
    use super::*;
    use std::thread;
    use std::sync::Arc;

    #[test]
    fn empty_try(){
        let mut q: AsyncQueue<i32> = AsyncQueue::new();
        assert_eq!(q.try_deq(), None);
    }

    #[test]
    fn add_remove_2_threads(){
        let q = Arc::new(AsyncQueue::new());
        let jh = {
            let q = q.clone();
            thread::spawn(move||{
                for x in 0..100{
                    q.enq(x);
                }
            })
        };
        let jh2 = {
            let q = q.clone();
            thread::spawn(move||{
                for x in 0..100{
                    let a = q.deq();
                    if x != a {
                        return false;
                    }
                }
                true
            })
        };
        jh.join().unwrap();
        assert!(jh2.join().unwrap());
    }
}
