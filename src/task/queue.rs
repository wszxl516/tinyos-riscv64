use core::ops::{Deref, DerefMut};

use alloc::vec::Vec;
pub struct TaskQueue<T> {
    queue: Vec<T>,
    current_index: Option<usize>,
}

impl<T> TaskQueue<T> {
    pub const fn new() -> Self {
        Self {
            queue: Vec::new(),
            current_index: None,
        }
    }
    pub fn next(&mut self) -> &mut T {
        let next = match &mut self.current_index {
            None => *self.current_index.get_or_insert(0),
            Some(next) => {
                *next += 1;
                if *next >= self.queue.len() {
                    *next = 0;
                };
                *next
            }
        };
        &mut self.queue[next]
    }
    pub fn current_mut(&mut self) -> Option<&mut T> {
        match self.current_index {
            Some(index) => Some(&mut self.queue[index]),
            None => None,
        }
    }
    pub fn current(&self) -> Option<&T> {
        match self.current_index {
            Some(index) => Some(&self.queue[index]),
            None => None,
        }
    }
    pub fn add(&mut self, t: T) {
        self.queue.push(t);
    }
}

impl<T> Deref for TaskQueue<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.queue
    }
}

impl<T> DerefMut for TaskQueue<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.queue
    }
}
