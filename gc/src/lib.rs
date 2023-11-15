#![forbid(unsafe_code)]

pub use gc_derive::Scan;

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    marker::PhantomData,
    ops::Deref,
    rc::{Rc, Weak},
};

////////////////////////////////////////////////////////////////////////////////

pub struct Gc<T> {
    weak: Weak<T>,
}

impl<T> Clone for Gc<T> {
    fn clone(&self) -> Self {
        Self {
            weak: self.weak.clone(),
        }
    }
}

impl<T> Gc<T> {
    pub fn address(&self) -> usize {
        self.weak.as_ptr() as usize
    }
    pub fn borrow<'a>(&self) -> GcRef<'a, T> {
        GcRef {
            rc: self.weak.upgrade().unwrap(),
            lifetime: PhantomData::<&'a Gc<T>>,
        }
    }
}

pub struct GcRef<'a, T> {
    rc: Rc<T>,
    lifetime: PhantomData<&'a Gc<T>>,
}

impl<'a, T> Deref for GcRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.rc
    }
}

////////////////////////////////////////////////////////////////////////////////

pub trait Scan {
    fn scan(&self) -> Vec<usize>;
}

impl<T: Scan + 'static> Scan for Gc<T> {
    fn scan(&self) -> Vec<usize> {
        vec![self.address()]
    }
}

impl<T: Scan + 'static> Scan for Vec<T> {
    fn scan(&self) -> Vec<usize> {
        let mut answer: Vec<usize> = vec![];
        for item in self.iter() {
            answer.append(&mut item.scan());
        }
        answer
    }
}

impl<T: Scan + 'static> Scan for Option<T> {
    fn scan(&self) -> Vec<usize> {
        if let Some(val) = self.as_ref() {
            return val.scan();
        }
        vec![]
    }
}

impl<T: Scan + 'static> Scan for RefCell<T> {
    fn scan(&self) -> Vec<usize> {
        self.borrow().scan()
    }
}

impl Scan for i32 {
    fn scan(&self) -> Vec<usize> {
        vec![]
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct Arena {
    allocs: Vec<Rc<dyn Scan + 'static>>,
    nodes: Vec<Vec<usize>>,
}

impl Default for Arena {
    fn default() -> Self {
        Self::new()
    }
}

impl Arena {
    pub fn new() -> Self {
        Self {
            allocs: Vec::<Rc<dyn Scan + 'static>>::new(),
            nodes: vec![vec![]],
        }
    }

    pub fn allocation_count(&self) -> usize {
        self.allocs.len()
    }

    pub fn alloc<T: Scan + 'static>(&mut self, obj: T) -> Gc<T> {
        let ptr: Rc<T> = Rc::new(obj);
        let w_ptr = Rc::<T>::downgrade(&ptr);
        self.allocs.push(ptr);
        Gc { weak: w_ptr }
    }

    pub fn sweep(&mut self) {
        let ptr_to_uint = HashMap::<usize, usize>::from_iter(
            (0..self.allocation_count())
                .map(|i| (Rc::as_ptr(&self.allocs[i]) as *const usize as usize, i)),
        );
        let mut cnt =
            HashMap::<usize, usize>::from_iter((0..self.allocation_count()).map(|i| (i, 0)));
        self.nodes.clear();
        for alloc in self.allocs.iter() {
            let mut line = vec![];
            for v in alloc.scan() {
                let res = ptr_to_uint[&v];
                cnt.entry(res).and_modify(|elem| *elem += 1);
                line.push(res);
            }
            self.nodes.push(line);
        }
        let mut marked = HashSet::<usize>::new();
        for (index, value) in cnt {
            if Rc::weak_count(&self.allocs[index]) > value {
                self.mark_all(index, &mut marked);
            }
        }
        let mut limit = 0;
        for i in 0..self.allocation_count() {
            if marked.contains(&i) {
                if i > limit {
                    self.allocs.swap(limit, i);
                }
                limit += 1;
            }
        }
        self.allocs.truncate(limit);
    }

    fn mark_all(&self, root_addr: usize, marked: &mut HashSet<usize>) {
        if marked.insert(root_addr) {
            for node in self.nodes[root_addr].iter() {
                self.mark_all(*node, marked);
            }
        }
    }
}
