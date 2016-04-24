#![allow(dead_code,unused_variables,unused_imports)]
use std::cmp::Ordering;
use std::cell::Cell;

#[derive(Clone, PartialEq)]
pub struct Clock {
    pub value: Cell<u32>,
}

impl Clock {
    pub fn new() -> Clock {
        Clock { value: Cell::new(1)}
    }
    pub fn increment(&self) {
        self.value.set(self.value.get() + 1);
    }
}

impl PartialOrd<Clock> for Clock {
    fn lt(&self, other: &Clock) -> bool {
        return self.value.get() < other.value.get();
    }

    fn partial_cmp(&self, other: &Clock) -> Option<Ordering> {
        if self.value.get() < other.value.get() {
            return Some(Ordering::Less);
        } else if self.value.get() < other.value.get() {
            return Some(Ordering::Greater);
        }
        return Some(Ordering::Equal);
    }
}
