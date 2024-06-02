#![forbid(unsafe_code)]

use std::{
    cell::RefCell,
    collections::VecDeque,
    fmt::Debug,
    rc::{Rc, Weak},
};

use thiserror::Error;

////////////////////////////////////////////////////////////////////////////////

#[derive(Error, Debug)]
#[error("channel is closed")]
pub struct SendError<T: Debug> {
    pub value: T,
}

pub struct CommonState<T> {
    queue: VecDeque<T>,
    alive: bool,
}

impl<T> Default for CommonState<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> CommonState<T> {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            alive: true,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn close(&mut self) {
        self.alive = false;
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn push_back(&mut self, val: T) {
        self.queue.push_back(val);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.queue.pop_front()
    }
}

pub struct Sender<T> {
    q: Rc<RefCell<CommonState<T>>>,
}

impl<T: Debug> Sender<T> {
    pub fn send(&self, value: T) -> Result<(), SendError<T>> {
        if self.is_closed() {
            return Err(SendError { value });
        }

        let mut b = self.q.borrow_mut();
        if !b.is_alive() {
            return Err(SendError { value });
        }
        b.push_back(value);
        Ok(())
    }

    pub fn is_closed(&self) -> bool {
        if Rc::weak_count(&self.q) == 0 {
            return true;
        }
        !self.q.borrow().is_alive()
    }

    pub fn same_channel(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.q, &other.q)
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Self { q: self.q.clone() }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Error, Debug)]
pub enum ReceiveError {
    #[error("channel is empty")]
    Empty,
    #[error("channel is closed")]
    Closed,
}

pub struct Receiver<T> {
    q: Weak<RefCell<CommonState<T>>>,
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Result<T, ReceiveError> {
        let opt_state = self.q.upgrade();
        if opt_state.is_none() {
            return Err(ReceiveError::Closed);
        }
        let state = opt_state.unwrap();
        let mut b = state.borrow_mut();
        if !b.is_empty() {
            return Ok(b.pop_front().unwrap());
        }
        if !b.is_alive() {
            return Err(ReceiveError::Closed);
        }
        Err(ReceiveError::Empty)
    }

    pub fn close(&mut self) {
        if let Some(state) = self.q.upgrade() {
            let mut b = state.borrow_mut();
            b.close()
        }
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        self.close()
    }
}

////////////////////////////////////////////////////////////////////////////////

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let state = Rc::new(RefCell::new(CommonState::new()));
    (
        Sender { q: state.clone() },
        Receiver {
            q: Rc::downgrade(&state),
        },
    )
}
