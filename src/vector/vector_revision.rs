use std::collections::tree_map::TreeMap;
use std::rc::Rc;
use std::vec::Vec;
use std::num::abs_sub;
use vector::fat_node::VectorElement;



pub struct VectorRevision {
    pub rev: i64,
    pub ary: Vec<Rc<int>>,
}

impl VectorRevision {
    pub fn push(&mut self, value: int) -> i64 {
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

    pub fn modify(&self, value: int) {
        assert!(false, "Not implemented");
    }

    pub fn iter(&self) {
        // TODO implement
    }

    pub fn get<'a>(&'a self, id: uint) -> &'a int {
        self.ary[id].deref()
    }
}

impl Index<uint, int> for VectorRevision {
    fn index<'a>(&'a self, id: &uint) -> &'a int {
        self.ary[*id].deref()
    }
}
