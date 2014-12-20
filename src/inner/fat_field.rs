use std::rc::Rc;
use std::cell::RefCell;
use std::collections::TreeMap;
use std::collections::HashMap;
use std::vec::Vec;
use inner::persistent::Revision;
use inner::lcg_random::*;

pub struct RevisionTree {
    generator: CoolLCG,
    root:      Revision,
    parent:    TreeMap<Revision, Revision>,
    history:   Vec<Revision>
}

impl RevisionTree {
    pub fn new() -> RevisionTree {
        let mut generator: CoolLCG = LCG::new();
        let root = generator.next();
        let history = vec![root];
        RevisionTree {
            generator: generator,
            root:      root,
            parent:    TreeMap::new(),
            history:   history
        }
    }

    fn parent(&self, r: Revision) -> Revision {
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

    pub fn history(&self) -> &Vec<Revision> {
        &self.history
    }

    pub fn root(&self) -> Revision {
        self.root
    }

    pub fn is_root(&self, r: Revision) -> bool {
        self.root == r
    }

    pub fn last(&self) -> Revision {
        self.history[self.last_index()]
    }

    pub fn last_index(&self) -> uint {
        self.history.len() - 1
    }

    pub fn fork(&mut self, parent: Revision) -> Revision {
        assert!(self.is_root(parent) || self.parent.contains_key(&parent));

        let child = self.generator.next();
        self.parent.insert(child, parent);
        self.history.push(child);
        child
    }
}

pub struct FatField<A> {
    values:  Rc<RefCell<HashMap<Revision, A>>>,
    tree:    Rc<RefCell<RevisionTree>>,
}

pub struct FatRef<'a, A: 'a> {
    value:      &'a A,
    head_index: Rc<RefCell<uint>>,
    values:     Rc<RefCell<HashMap<Revision, A>>>,
    tree:       Rc<RefCell<RevisionTree>>
}

impl<'a, A: 'a> FatRef<'a, A> {
    pub fn map(&self, f: |&'a A| -> A) {
        let h = self.tree.borrow().history()[*self.head_index.borrow()];
        let r = self.tree.borrow_mut().fork(h);
        *self.head_index.borrow_mut() = self.tree.borrow().last_index();
        self.values.borrow_mut().insert(r, f(self.value));
    }
}

impl<'a, A: 'a> Deref<A> for FatRef<'a, A> {
    fn deref(&self) -> &A {
        self.value
    }
}

impl<A> FatField<A> {
    pub fn new(tree: Rc<RefCell<RevisionTree>>) -> FatField<A> {
        FatField {
            values: Rc::new(RefCell::new(HashMap::new())),
            tree:   tree
        }
    }

    pub fn get<'a>(&self, r: Revision) -> Option<&'a A> {
        for c in self.tree.borrow().parent_branch(r).iter() {
            match self.values.borrow().get(c) {
                None         => continue,
                Some(&ref v) => return Some(v)
            }
        }
        None
    }

    pub fn get_fat_ref<'a>(&self, r: Revision, i: Rc<RefCell<uint>>) -> Option<FatRef<'a, A>> {
        for c in self.tree.borrow().parent_branch(r).iter() {
            match self.values.borrow().get(c) {
                None         => continue,
                Some(&ref value) => {
                    return Some(FatRef{
                        value:      value,
                        head_index: i,
                        values:     self.values.clone(),
                        tree:       self.tree.clone()
                    })
                }
            }
        }
        None
    }

    pub fn insert(&mut self, r: Revision, v: A) {
        self.values.borrow_mut().insert(r, v);
    }
}

#[cfg(test)]
fn magic_tree() -> (RevisionTree, Vec<Revision>) {
    let mut t = RevisionTree::new();

    //           6
    //          /
    //         /
    //  0--1--3--5
    //      \
    //       \
    //        2--4--7

    let p = vec![0, 1, 1, 2, 3, 3, 4];
    let mut h = vec![t.root()];
    for &i in p.iter() {
        let n = h[i];
        h.push(t.fork(n));
    }

    (t, h)
}

#[test]
fn revision_tree() {
    let (t, h) = magic_tree();

    assert_eq!(t.parent(h[2]), h[1]);
    assert_eq!(t.parent(h[3]), h[1]);
    assert_eq!(t.parent(h[4]), h[2]);
    assert_eq!(t.parent(h[7]), h[4]);

    assert_eq!(t.parent_branch(h[0]), vec![h[0]]);
    assert_eq!(t.parent_branch(h[1]), vec![h[1], h[0]]);
    assert_eq!(t.parent_branch(h[6]), vec![h[6], h[3], h[1], h[0]]);

    assert!( t.is_root(h[0]));
    assert!(!t.is_root(h[1]));
    assert!(!t.is_root(h[4]));
}

#[test]
fn fat_field() {
    let (t, h) = magic_tree();
    let mut a: FatField<&str> = FatField::new(Rc::new(RefCell::new(t)));

    a.insert(h[1], "1");
    a.insert(h[2], "2");
    a.insert(h[3], "3");
    a.insert(h[4], "4");
    a.insert(h[5], "5");
    a.insert(h[6], "6");
    a.insert(h[7], "7");

    assert_eq!(a.get(h[2]), Some(&"2"));
    assert_eq!(a.get(h[5]), Some(&"5"));
    assert_eq!(a.get(h[7]), Some(&"7"));
}

#[test]
fn multiple_fat_fields() {
    let (_t, h) = magic_tree();
    let t = Rc::new(RefCell::new(_t));

    let mut a = FatField::new(t.clone());
    let mut b = FatField::new(t.clone());
    let mut c = FatField::new(t);

    a.insert(h[1], "1");
    a.insert(h[2], "2");
    a.insert(h[7], "7");

    b.insert(h[4], "4");
    b.insert(h[5], "5");

    c.insert(h[3], "3");
    c.insert(h[6], "6");

    assert_eq!(a.get(h[7]), Some(&"7"));
    assert_eq!(b.get(h[7]), Some(&"4"));
    assert_eq!(c.get(h[7]), None);

    assert_eq!(a.get(h[4]), Some(&"2"));
    assert_eq!(b.get(h[4]), Some(&"4"));
    assert_eq!(c.get(h[4]), None);

    assert_eq!(a.get(h[5]), Some(&"1"));
    assert_eq!(b.get(h[5]), Some(&"5"));
    assert_eq!(c.get(h[5]), Some(&"3"));
}
