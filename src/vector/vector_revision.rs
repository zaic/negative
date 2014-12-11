use std::collections::tree_map::TreeMap;
use std::rc::Rc;
use std::vec::Vec;
use std::num::abs_sub;
use vector::fat_node::VectorElement;



pub struct VectorRevision<T> {
    pub rev: i64,
    pub ary: Vec<Rc<T>>,
}

impl<T> VectorRevision<T> {
    pub fn push(&mut self, value: T) -> i64 {
        assert!(false, "Not implemented");
        -1
    }

    pub fn pop(&mut self) -> i64 {
        assert!(false, "Not implemented");
        -1
    }

    pub fn len(&self) -> uint {
        self.ary.len()
    }

    pub fn iter(&self) {
        // TODO implement
    }

    pub fn get<'a>(&'a self, id: uint) -> &'a T {
        self.ary[id].deref()
    }

    pub fn modify(&mut self, id: uint, value: T) {
        assert!(false, "Not implemented");
    }
}

impl<T> Index<uint, T> for VectorRevision<T> {
    fn index<'a>(&'a self, id: &uint) -> &'a T {
        self.ary[*id].deref()
    }
}
