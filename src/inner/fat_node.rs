/*
 *  This file contains generic FatNode implementation.
 *
 *  Implementation use search trees and allow access to specific revision with
 *  O(lg(N)) time and inserting new revision with O(lg(N)) time.
 */

use std::collections::BTreeMap as TreeMap;
use std::rc::Rc;
use inner::persistent::Revision;

pub struct FatNode<T> {
    history: TreeMap<Revision, Option<Rc<T>>>
}

impl<T> FatNode<T> {
    pub fn new() -> FatNode<T> {
        let mut map = TreeMap::new();
        map.insert(0, None);
        FatNode{history: map}
    }

    pub fn value<'a>(&'a self, revision: Revision) -> Option<Rc<T>> {
        match self.history.iter().rposition(|(&k, _)| k <= revision) {
            None    => None,
            Some(i) =>
                match *self.history.iter().skip(i).next().unwrap().1 {
                    None          => None,
                    Some(ref rct) => Some(rct.clone()),
                }
        }
    }

    pub fn add_value(&mut self, revision: Revision, value: Option<T>) {
        match value {
            None =>
                self.history.insert(revision, None),
            Some(real_value) =>
                self.history.insert(revision, Some(Rc::new(real_value)))
        };
    }
}

#[test]
fn fatnode_init_first_test() {
    let mut node = FatNode::<int>::new();
    assert_eq!(node.value(0), None);
    assert_eq!(node.value(1), None);
    node.add_value(1, Some(1807));
    assert_eq!(node.value(0), None);
    assert_eq!(*node.value(1).unwrap(), 1807);
}

#[test]
fn fatnode_init_second_test() {
    let mut node2 = FatNode::<int>::new();
    node2.add_value(1, Some(-10));
    node2.add_value(10, Some(12));
    assert_eq!(node2.value(0), None);
    assert_eq!(*node2.value(1).unwrap(), -10);
    assert_eq!(*node2.value(2).unwrap(), -10);
    assert_eq!(*node2.value(9).unwrap(), -10);
    assert_eq!(*node2.value(10).unwrap(), 12);
    assert_eq!(*node2.value(11).unwrap(), 12);
}

#[test]
fn fatnode_generic_string_test() {
    let mut node_str = FatNode::<&str>::new();
    node_str.add_value(1, Some("hello"));
    node_str.add_value(10, Some("rust"));
    assert_eq!(node_str.value(0), None);
    assert_eq!(*node_str.value(1).unwrap(), "hello");
    assert_eq!(*node_str.value(2).unwrap(), "hello");
    assert_eq!(*node_str.value(9).unwrap(), "hello");
    assert_eq!(*node_str.value(10).unwrap(), "rust");
    assert_eq!(*node_str.value(11).unwrap(), "rust");
}

#[test]
fn fatnode_generic_float_test() {
    use std::num::FloatMath;

    let mut node_flt = FatNode::<f32>::new();
    node_flt.add_value(12, Some(-1.0));
    node_flt.add_value(13, Some(1e-7));
    node_flt.add_value(14, Some(1.3333333));

    let eps = 1e-8f32;
    assert_eq!(node_flt.value(11), None);
    assert!(FloatMath::abs_sub(*node_flt.value(12).unwrap(), -1.0) < eps);
    assert!(FloatMath::abs_sub(*node_flt.value(13).unwrap(), 1e-7) < eps);
    assert!(FloatMath::abs_sub(*node_flt.value(14).unwrap(), 1.3333333) < eps);
    assert!(FloatMath::abs_sub(*node_flt.value(15).unwrap(), 1.3333333) < eps);
}
