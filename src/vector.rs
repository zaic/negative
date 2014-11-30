use std::collections::tree_map::TreeMap;
use std::rc::Rc;
use std::vec::Vec;



struct VectorElement<T> {
    history: TreeMap<i64, Rc<Option<T>>>,
}

impl<T> VectorElement<T> {
    pub fn new() -> VectorElement<T> {
        let mut map = TreeMap::new();
        map.insert(0, Rc::new(None));
        VectorElement{history: map}
    }

    pub fn value(&self, revision: i64) -> Rc<Option<T>> {
        // TODO improve performance usinb binary search
        //   common algo: use upper_bound() and move iterator by one element back
        //   but Iterator in rust doesn't have methods suck back() or next_rev()
        let mut iter = self.history.rev_iter();
        loop {
            let (key, val) = match iter.next() {
                Some((key, val)) => (*key, (*val).clone()),
                None             => (0i64, Rc::new(None)),
            };
            if key <= revision {
                return val.clone(); // TODO remove clone?
            }
        }
    }

    pub fn add_value(&mut self, revision: i64, value: Option<T>) {
        self.history.insert(revision, Rc::new(value));
    }
}

#[test]
fn element_init() {
    let mut node = VectorElement::<int>::new();
    assert_eq!(*node.value(0i64), None);
    assert_eq!(*node.value(1i64), None);
    node.add_value(1i64, Some(1807));
    assert_eq!(*node.value(0i64), None);
    assert_eq!(*node.value(1i64), Some(1807));

    let mut node2 = VectorElement::<int>::new();
    node2.add_value(1, Some(-10));
    node2.add_value(10, Some(12));
    assert_eq!(*node2.value(0), None);
    assert_eq!(*node2.value(1), Some(-10));
    assert_eq!(*node2.value(2), Some(-10));
    assert_eq!(*node2.value(9), Some(-10));
    assert_eq!(*node2.value(10), Some(12));
    assert_eq!(*node2.value(11), Some(12));
}

/*
#[test]
fn element_modify() {
    let mut node = VectorElement::new();
    node.add_value(1, "hello");
    node.add_value(10, "rust");
    assert_eq!(node.value(0), None);
    assert_eq!(node.value(1), "hello");
    assert_eq!(node.value(2), "hello");
    assert_eq!(node.value(9), "hello");
    assert_eq!(node.value(10), "rust");
    assert_eq!(node.value(11), "rust");
}
*/




struct VectorRevision {
    rev: i64,
    ary: Vec<Rc<int>>,
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

    pub fn get(&self, id: uint) -> int {
        *self.ary[id]
    }
}

impl Index<uint, int> for VectorRevision {
    fn index(&self, id: &uint) -> &int {
        &*self.ary[*id]
    }
}



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
}
