use std::collections::BTreeMap as TreeMap;
use std::collections::HashMap;
use std::vec::Vec;
use inner::persistent::Revision;
use inner::lcg_random::*;
use std::cell::*;

pub type Field<A> = HashMap<Revision, A>;

pub struct Tree {
    generator: CoolLCG,
    root:      Revision,
    parent:    TreeMap<Revision, Revision>,
    history:   Vec<Revision>
}

impl Tree {
    pub fn new() -> Tree {
        let mut generator: CoolLCG = LCG::new();
        let root = generator.next();
        let history = vec![root];
        Tree {
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

    fn ancestors(&self, r: Revision) -> Vec<Revision> {
        assert!(r == self.root || self.parent.contains_key(&r));

        let mut b = vec!(r);
        let mut c = r;
        while c != self.root {
            c = self.parent(c);
            b.push(c);
        }
        b
    }

    pub fn root(&self) -> Revision {
        self.root
    }

    pub fn revision(&self, i: uint) -> Revision {
        self.history[i]
    }

    pub fn last_index(&self) -> uint {
        self.history.len() - 1
    }

    pub fn fork(&mut self, p: Revision) -> Revision {
        assert!(p == self.root || self.parent.contains_key(&p));

        let c = self.generator.next();
        self.parent.insert(c, p);
        self.history.push(c);
        c
    }

    pub fn _get<'a, A>(&self, m: Ref<'a, Field<A>>, r: Revision) -> Option<&'a A> {
        for c in self.ancestors(r).iter() {
            match m.get(c) {
                None         => continue,
                Some(&ref v) => return Some(v),
            }
        }
        None
    }

    pub fn get<'a, A>(&self, m: &Field<A>, r: Revision) -> Option<&'a A> {
        for c in self.ancestors(r).iter() {
            match m.get(c) {
                None         => continue,
                Some(&ref v) => return Some(v),
            }
        }
        None
    }
}

#[cfg(test)]
fn magic_tree() -> Tree {
    let mut t = Tree::new();

    //           6
    //          /
    //         /
    //  0--1--3--5
    //      \
    //       \
    //        2--4--7

    let p = vec![0, 1, 1, 2, 3, 3, 4];
    for &i in p.iter() {
        let n = t.history[i];
        t.fork(n);
    }

    t
}

#[test]
fn revision_tree() {
    let t = magic_tree();
    let h = &t.history;

    assert_eq!(t.ancestors(h[0]), vec![h[0]]);
    assert_eq!(t.ancestors(h[1]), vec![h[1], h[0]]);
    assert_eq!(t.ancestors(h[6]), vec![h[6], h[3], h[1], h[0]]);
}

#[test]
fn fat_field() {
    let t = magic_tree();
    let h = &t.history;
    let mut a: Field<&str> = HashMap::new();

    a.insert(h[1], "1");
    a.insert(h[2], "2");
    a.insert(h[3], "3");
    a.insert(h[4], "4");
    a.insert(h[5], "5");
    a.insert(h[6], "6");
    a.insert(h[7], "7");

    assert_eq!(t.get(&a, h[2]), Some(&"2"));
    assert_eq!(t.get(&a, h[5]), Some(&"5"));
    assert_eq!(t.get(&a, h[7]), Some(&"7"));
}

#[test]
fn multiple_fat_fields() {
    let t = magic_tree();
    let h = &t.history;

    let mut a = HashMap::new();
    let mut b = HashMap::new();
    let mut c = HashMap::new();

    a.insert(h[1], "1");
    a.insert(h[2], "2");
    a.insert(h[7], "7");

    b.insert(h[4], "4");
    b.insert(h[5], "5");

    c.insert(h[3], "3");
    c.insert(h[6], "6");

    assert_eq!(t.get(&a, h[7]), Some(&"7"));
    assert_eq!(t.get(&b, h[7]), Some(&"4"));
    assert_eq!(t.get(&c, h[7]), None);

    assert_eq!(t.get(&a, h[4]), Some(&"2"));
    assert_eq!(t.get(&b, h[4]), Some(&"4"));
    assert_eq!(t.get(&c, h[4]), None);

    assert_eq!(t.get(&a, h[5]), Some(&"1"));
    assert_eq!(t.get(&b, h[5]), Some(&"5"));
    assert_eq!(t.get(&c, h[5]), Some(&"3"));
}
