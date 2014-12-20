use std::rc::Rc;
use std::cell::RefCell;
use std::collections::TreeMap;
use std::collections::HashMap;
use std::vec::Vec;
use inner::persistent::Revision;

pub struct RevisionTree {
    root:   Revision,
    parent: TreeMap<Revision, Revision>,
}

impl RevisionTree {
    pub fn new(r: Revision) -> RevisionTree {
        RevisionTree {
            root:   r,
            parent: TreeMap::new()
        }
    }

    pub fn parent(&self, r: Revision) -> Revision {
        assert!(self.parent.contains_key(&r));

        self.parent[r]
    }

    pub fn parent_branch(&self, r: Revision) -> Vec<Revision> {
        assert!(self.is_root(r) || self.parent.contains_key(&r));

        let mut b = vec!(r);
        let mut c = r;
        while !self.is_root(c) {
            c = self.parent(c);
            b.push(c);
        }
        b
    }

    pub fn is_root(&self, r: Revision) -> bool {
        self.root == r
    }

    pub fn insert(&mut self, c: Revision, p: Revision) {
        assert!(!self.parent.contains_key(&c));
        assert!(self.is_root(p) || self.parent.contains_key(&p));

        self.parent.insert(c, p);
    }
}

pub struct FatField<A> {
    values:        HashMap<Revision, A>,
    revision_tree: Rc<RefCell<RevisionTree>>
}

impl<A> FatField<A> {
    pub fn new(rt: Rc<RefCell<RevisionTree>>) -> FatField<A> {
        FatField {
            values:        HashMap::new(),
            revision_tree: rt
        }
    }

    pub fn get(&self, r: Revision) -> Option<&A> {
        for c in self.revision_tree.borrow().parent_branch(r).iter() {
            match self.values.get(c) {
                None         => continue,
                Some(&ref v) => return Some(v)
            }
        }
        None
    }

    pub fn insert(&mut self, r: Revision, v: A) {
        self.values.insert(r, v);
    }
}

#[cfg(test)]
fn magic_tree() -> RevisionTree{
    let mut a = RevisionTree::new(1);

    //           7
    //          /
    //         /
    //  1--2--4--6
    //      \
    //       \
    //        3--5--8

    a.insert(2, 1);
    a.insert(3, 2);
    a.insert(4, 2);
    a.insert(5, 3);
    a.insert(6, 4);
    a.insert(7, 4);
    a.insert(8, 5);

    a
}

#[test]
fn revision_tree() {
    let a = magic_tree();

    assert_eq!(a.parent(2), 1);
    assert_eq!(a.parent(3), 2);
    assert_eq!(a.parent(4), 2);
    assert_eq!(a.parent(8), 5);

    assert_eq!(a.parent_branch(1), vec![1]);
    assert_eq!(a.parent_branch(2), vec![2, 1]);
    assert_eq!(a.parent_branch(7), vec![7, 4, 2, 1]);

    assert!( a.is_root(1));
    assert!(!a.is_root(2));
    assert!(!a.is_root(5));
}


#[test]
fn fat_field() {
    let rt = Rc::new(RefCell::new(magic_tree()));

    let mut a: FatField<&str> = FatField::new(rt.clone());

    a.insert(2, "2");
    a.insert(3, "3");
    a.insert(4, "4");
    a.insert(5, "5");
    a.insert(6, "6");
    a.insert(7, "7");
    a.insert(8, "8");

    assert_eq!(a.get(2), Some(&"2"));
    assert_eq!(a.get(5), Some(&"5"));
    assert_eq!(a.get(7), Some(&"7"));
}

#[test]
fn multiple_fat_fields() {
    let rt = Rc::new(RefCell::new(magic_tree()));

    let mut a = FatField::new(rt.clone());
    let mut b = FatField::new(rt.clone());
    let mut c = FatField::new(rt.clone());

    a.insert(2, "2");
    a.insert(3, "3");
    a.insert(8, "8");

    b.insert(5, "5");
    b.insert(6, "6");

    c.insert(4, "4");
    c.insert(7, "7");

    assert_eq!(a.get(8), Some(&"8"));
    assert_eq!(b.get(8), Some(&"5"));
    assert_eq!(c.get(8), None);

    assert_eq!(a.get(5), Some(&"3"));
    assert_eq!(b.get(5), Some(&"5"));
    assert_eq!(c.get(5), None);

    assert_eq!(a.get(6), Some(&"2"));
    assert_eq!(b.get(6), Some(&"6"));
    assert_eq!(c.get(6), Some(&"4"));
}
