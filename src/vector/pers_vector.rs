use std::collections::tree_map::TreeMap;
use std::rc::Rc;
use std::vec::Vec;
use std::num::abs_sub;
use vector::fat_node::VectorElement;
use vector::vector_revision::VectorRevision;



struct PersVector {
    rev: i64,

    ary: Vec<VectorElement<int>>,
    len: uint,
}

impl PersVector {
    pub fn new() -> PersVector {
        PersVector{rev: 0, ary: Vec::new(), len: 0}
    }

    pub fn get_by_revision(&self, revision : i64) -> VectorRevision {
        assert!(revision <= self.rev);
        let mut result_vector = Vec::<Rc<int>>::new();
        // TODO use iterator ;)
        for i in range(0u, self.ary.len()) {
            match *self.ary[i].value(revision) {
                Some(value) => result_vector.push(Rc::new(value)),
                None        => break,
            };
        }
        VectorRevision{rev: revision, ary: result_vector}
    }



    pub fn len(&self) -> uint {
        self.len
    }

    pub fn push(&mut self, value: int) -> i64 {
        self.rev += 1;
        if self.ary.len() == self.len {
            self.ary.push(VectorElement::new());
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
}

#[test]
fn vec_test() {
    let mut v = PersVector::new();
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
