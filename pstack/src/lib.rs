#![forbid(unsafe_code)]

use std::rc::Rc;

pub struct PRef<T> {
    data: Rc<T>,
    prev: Option<Rc<PRef<T>>>,
}

impl<T> PRef<T> {
    pub fn new(data: T) -> Self {
        PRef {
            data: Rc::new(data),
            prev: None,
        }
    }
}

impl<T> Clone for PRef<T> {
    fn clone(&self) -> Self {
        PRef {
            data: self.data.clone(),
            prev: self.prev.clone(),
        }
    }
}

impl<T> std::ops::Deref for PRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

pub struct PStack<T> {
    head: Option<Rc<PRef<T>>>,
    size: usize,
}

impl<T> Default for PStack<T> {
    fn default() -> Self {
        PStack {
            head: None,
            size: 0,
        }
    }
}

impl<T> Clone for PStack<T> {
    fn clone(&self) -> Self {
        PStack {
            head: self.head.clone(),
            size: self.size,
        }
    }
}

impl<T> PStack<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&self, value: T) -> Self {
        let new_node = PRef {
            data: Rc::new(value),
            prev: self.head.clone(),
        };
        PStack {
            head: Some(Rc::new(new_node)),
            size: self.size + 1,
        }
    }

    pub fn pop(&self) -> Option<(PRef<T>, Self)> {
        match self.head.as_ref() {
            Some(node) => {
                let new_stack = PStack {
                    head: node.prev.clone(),
                    size: self.size - 1,
                };
                Some((
                    PRef {
                        data: node.data.clone(),
                        prev: node.prev.clone(),
                    },
                    new_stack,
                ))
            }
            None => None,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = PRef<T>> {
        PStackIterator {
            current: self.head.clone(),
        }
    }
}

struct PStackIterator<T> {
    current: Option<Rc<PRef<T>>>,
}

impl<T> Iterator for PStackIterator<T> {
    type Item = PRef<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current.clone() {
            Some(node) => {
                self.current = node.prev.clone();
                Some(PRef {
                    data: node.data.clone(),
                    prev: node.prev.clone(),
                })
            }
            None => None,
        }
    }
}
