use std::collections::tree_map::TreeMap;
use std::rc::Rc;
use std::vec::Vec;
use std::num::abs_sub;



pub struct VectorElement<T> {
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

#[test]
fn element_generic() {
    let mut node_str = VectorElement::<&str>::new();
    node_str.add_value(1, Some("hello"));
    node_str.add_value(10, Some("rust"));
    assert_eq!(*node_str.value(0), None);
    assert_eq!(*node_str.value(1), Some("hello"));
    assert_eq!(*node_str.value(2), Some("hello"));
    assert_eq!(*node_str.value(9), Some("hello"));
    assert_eq!(*node_str.value(10), Some("rust"));
    assert_eq!(*node_str.value(11), Some("rust"));

    let mut node_flt = VectorElement::<f32>::new();
    node_flt.add_value(12, Some(-1.0));
    node_flt.add_value(13, Some(1e-7));
    node_flt.add_value(14, Some(1.3333333));
    assert_eq!(*node_flt.value(11), None); // TODO check with abs and eps
    assert!(abs_sub((*node_flt.value(12)).unwrap_or(10.0), -1.0) < 1e-8);
    assert!(abs_sub((*node_flt.value(13)).unwrap_or(10.0), 1e-7) < 1e-8);
    assert!(abs_sub((*node_flt.value(14)).unwrap_or(10.0), 1.3333333) < 1e-8);
    assert!(abs_sub((*node_flt.value(15)).unwrap_or(10.0), 1.3333333) < 1e-8);
}
