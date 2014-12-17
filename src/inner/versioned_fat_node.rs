/*
 *  This file contains generic FatNode implementation for structures, which
 *  supports undo-redo.
 */

use std::cell::RefCell;
use std::collections::tree_map::TreeMap;
use std::rc::Rc;
use std::vec::Vec;



pub struct VersionTree {
    parent: TreeMap<i64, i64>,
}

impl VersionTree {
    pub fn new(initial_revision: i64) -> VersionTree {
        let mut p = TreeMap::new();
        p.insert(initial_revision, -1);
        VersionTree{parent: p}
    }

    pub fn parent_revision(&self, revision: i64) -> i64 {
        assert!(revision > 0);
        assert!(self.parent.contains_key(&revision));

        self.parent[revision]
    }

    pub fn parent_branch(&self, revision: i64) -> Vec<i64> {
        let mut branch = vec![revision];
        loop {
            let cur_rev = branch[branch.len() - 1];
            if self.is_initial(cur_rev) {
                break;
            }
            branch.push(self.parent_revision(cur_rev));
        }
        branch
    }

    pub fn is_initial(&self, revision: i64) -> bool {
        assert!(revision > 0);
        assert!(self.parent.contains_key(&revision));

        self.parent[revision] == -1
    }

    pub fn insert(&mut self, new_revision: i64, old_revision: i64) {
        assert!(new_revision > 0);
        assert!(!self.parent.contains_key(&new_revision));
        assert!(old_revision == -1 || self.parent.contains_key(&old_revision));

        self.parent.insert(new_revision, old_revision);
    }
}

#[test]
fn version_tree_test() {
    /*
     *           7
     *          / 
     *         /
     *  1--2--4--6
     *      \
     *       \
     *        3--5--8
     */
    let mut vt = VersionTree::new(1);
    vt.insert(2, 1);
    vt.insert(3, 2);
    vt.insert(4, 2);
    vt.insert(5, 3);
    vt.insert(6, 4);
    vt.insert(7, 4);
    vt.insert(8, 5);

    assert_eq!(vt.parent_revision(2), 1);
    assert_eq!(vt.parent_revision(3), 2);
    assert_eq!(vt.parent_revision(4), 2);
    assert_eq!(vt.parent_revision(8), 5);
    assert_eq!(vt.parent_revision(1), -1);

    assert_eq!(vt.parent_branch(1), vec![1]);
    assert_eq!(vt.parent_branch(2), vec![2, 1]);
    assert_eq!(vt.parent_branch(7), vec![7, 4, 2, 1]);

    assert!(vt.is_initial(1));
    assert!(!vt.is_initial(2));
    assert!(!vt.is_initial(5));
}



pub struct VersionedFatNode<T> {
    values: TreeMap<i64, Option<Rc<T>>>, // TODO
                                         // lower_bound() no longer required
                                         // can be replaced by HashMap
    revisions: Rc<RefCell<VersionTree>>
}

impl<T> VersionedFatNode<T> {
    pub fn new(rev: Rc<RefCell<VersionTree>>) -> VersionedFatNode<T> {
        let mut map = TreeMap::new();
        map.insert(0, None);
        VersionedFatNode{values: map, revisions: rev.clone()}
    }

    pub fn value<'a>(&'a self, revision: i64) -> Option<Rc<T>> {
        for cur_rev in self.revisions.borrow().parent_branch(revision).iter() {
            match self.values.find(cur_rev) {
                None => continue,
                Some(ref value) =>
                    match *value {
                        &None =>
                            return None,
                        &Some(ref rct) =>
                            return Some(rct.clone())
                    }
            }
        }
        None
    }

    pub fn add_value(&mut self, new_revision: i64, real_value: Option<T>, old_revision: i64) {
        self.revisions.borrow_mut().insert(new_revision, old_revision);
        match real_value {
            None =>
                self.values.insert(new_revision, None),
            Some(real_value) =>
                self.values.insert(new_revision, Some(Rc::new(real_value))),
        };
    }
}

#[test]
fn versioned_fatnode_test() {
    let mut vs = VersionTree::new(1);
    let mut vf = VersionedFatNode::<&str>::new(Rc::new(RefCell::new(vs)));

    /*
     * Build the same tree as above:
     *
     *           7
     *          / 
     *         /
     *  1--2--4--6
     *      \
     *       \
     *        3--5--8
     */
    vf.add_value(2, Some("two"), 1);
    vf.add_value(3, Some("three"), 2);
    vf.add_value(4, Some("four"), 2);
    vf.add_value(5, Some("five"), 3);
    vf.add_value(6, Some("six"), 4);
    vf.add_value(7, Some("seven"), 4);
    vf.add_value(8, Some("eight"), 5);

    assert_eq!(*vf.value(2).unwrap(), "two");
    assert_eq!(*vf.value(5).unwrap(), "five");
    assert_eq!(*vf.value(7).unwrap(), "seven");
    assert_eq!(vf.value(1), None);
}

#[test]
fn three_fatnodes_test() {
    let mut vs = Rc::new(RefCell::new(VersionTree::new(1)));
    let mut vfa = VersionedFatNode::<&str>::new(vs.clone());
    let mut vfb = VersionedFatNode::<&str>::new(vs.clone());
    let mut vfc = VersionedFatNode::<&str>::new(vs.clone());

    /*
     * Build this tree one more time...
     *
     *           7c
     *          / 
     *         /
     *  1--2a--4c--6b
     *      \
     *       \
     *        3a--5b--8a
     */
    vfa.add_value(2, Some("two"), 1);
    vfa.add_value(3, Some("three"), 2);
    vfc.add_value(4, Some("four"), 2);
    vfb.add_value(5, Some("five"), 3);
    vfb.add_value(6, Some("six"), 4);
    vfc.add_value(7, Some("seven"), 4);
    vfa.add_value(8, Some("eight"), 5);

    assert_eq!(*vfa.value(8).unwrap(), "eight");
    assert_eq!(*vfb.value(8).unwrap(), "five");
    assert_eq!( vfc.value(8), None);

    assert_eq!(*vfa.value(5).unwrap(), "three");
    assert_eq!(*vfb.value(5).unwrap(), "five");
    assert_eq!( vfc.value(5), None);

    assert_eq!(*vfa.value(6).unwrap(), "two");
    assert_eq!(*vfb.value(6).unwrap(), "six");
    assert_eq!(*vfc.value(6).unwrap(), "four");
}
