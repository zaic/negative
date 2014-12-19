use std::rc::Rc;
use std::vec::Vec;
use inner::persistent::Revision;



pub struct VectorRevision<T> {
    pub rev: Revision,
    pub ary: Vec<Rc<T>>,
}

impl<T> VectorRevision<T> {
    pub fn push(&mut self, _: T) -> Revision {
        assert!(false, "Not implemented");
        -1
    }

    pub fn pop(&mut self) -> Revision {
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

    pub fn modify(&mut self, _: uint, _: T) {
        assert!(false, "Not implemented");
    }
}

impl<T> Index<uint, T> for VectorRevision<T> {
    fn index<'a>(&'a self, id: &uint) -> &'a T {
        self.ary[*id].deref()
    }
}
