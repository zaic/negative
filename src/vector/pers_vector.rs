use std::rc::Rc;
use std::vec::Vec;
use inner::fat_node::FatNode;
use inner::persistent::Persistent;
use vector::vector_revision::VectorRevision;



pub struct PersVector<T> {
    rev: i64,

    ary: Vec<FatNode<T>>,
    len: uint,
}

impl<T> Persistent<VectorRevision<T>> for PersVector<T> {
    fn get_by_revision(&self, revision : i64) -> VectorRevision<T> {
        assert!(revision <= self.rev);
        let mut result_vector = Vec::<Rc<T>>::new();
        // TODO use iterator ;)
        for i in range(0u, self.ary.len()) {
            match self.ary[i].value(revision) {
                Some(ref rct) => result_vector.push(rct.clone()),
                None      => break,
            };
        }
        VectorRevision{rev: revision, ary: result_vector}
    }

    fn current_revision(&self) -> i64 {
        self.rev
    }

    fn undo(&mut self) -> i64 {
        panic!("Not implemented");
    }

    fn redo(&mut self) -> i64 {
        panic!("Not implemented");
    }
}

impl<T> PersVector<T> {
    pub fn new() -> PersVector<T> {
        PersVector{rev: 0, ary: Vec::new(), len: 0}
    }



    pub fn len(&self) -> uint {
        self.len
    }

    pub fn push(&mut self, value: T) -> i64 {
        self.rev += 1;
        if self.ary.len() == self.len {
            self.ary.push(FatNode::new());
        }
        self.ary[self.len].add_value(self.rev, Some(value));
        self.len += 1;
        self.rev
    }

    pub fn pop(&mut self) -> i64 {
        assert!(self.len > 0);

        self.rev += 1;
        self.len -= 1;
        self.ary[self.len].add_value(self.rev, None);
        self.rev
    }

    pub fn modify(&mut self, id: uint, value: T) -> i64 {
        assert!(id < self.len);

        self.rev += 1;
        self.ary[id].add_value(self.rev, Some(value));
        self.rev
    }
}

#[test]
fn vec_test() {
    let mut v = PersVector::<int>::new();
    v.push(1807);
    let rev_before = v.push(2609);
    assert_eq!(v.len(), 2u);
    let rev_middle = v.pop();
    assert_eq!(v.len(), 1u);
    let rev_after = v.push(1008);
    assert_eq!(v.len(), 2u);

    let vec_before = v.get_by_revision(rev_before);
    assert_eq!(vec_before[0], 1807);
    assert_eq!(vec_before[1], 2609);
    assert_eq!(vec_before.len(), 2);

    let vec_middle = v.get_by_revision(rev_middle);
    assert_eq!(vec_middle[0], 1807);
    assert_eq!(vec_middle.len(), 1);

    let vec_after = v.get_by_revision(rev_after);
    assert_eq!(vec_after[0], 1807);
    assert_eq!(vec_after[1], 1008);
    assert_eq!(vec_after.len(), 2);

    let immut_val = vec_middle[0];
    assert_eq!(immut_val, 1807);
}

#[test]
fn vec_generic_test() {
    let mut vs = PersVector::<&str>::new();
    vs.push("pysch");
    vs.push("pysch");
    let rev_pysch = vs.push("pysch");
    assert_eq!(vs.len(), 3u);
    vs.pop();
    let rev_ololo = vs.push("ololo");
    assert_eq!(vs.get_by_revision(rev_pysch)[2], "pysch");
    assert_eq!(vs.get_by_revision(rev_ololo)[2], "ololo");
}

#[test]
fn vec_modify_test() {
    let mut v = PersVector::<int>::new();
    for i in range(0i, 10i) {
        v.push(i);
    }
    let pre_modify = v.last_revision();
    v.modify(7, 1807);
    let post_modify = v.last_revision();

    assert_eq!(pre_modify[7], 7);
    assert_eq!(post_modify[7], 1807);
}
