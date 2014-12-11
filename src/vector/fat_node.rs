use std::collections::tree_map::TreeMap;
use std::rc::Rc;
//use std::vec::Vec;
#[test]
use std::num::abs_sub;



pub struct VectorElement<T> {
    history: TreeMap<i64, Option<Rc<T>>>
}

impl<T> VectorElement<T> {
    pub fn new() -> VectorElement<T> {
        let mut map = TreeMap::new();
        map.insert(0, None);
        VectorElement{history: map}
    }

    pub fn value<'a>(&'a self, revision: i64) -> Option<Rc<T>> {
        // TODO improve performance usinb binary search
        //   common algo: use upper_bound() and move iterator by one element back
        //   but Iterator in rust doesn't have methods suck back() or next_rev()
        let mut iter = self.history.rev_iter();
        loop {
            let (key, val) = iter.next().unwrap();
            /*
            let (key, val) = match iter.next() {
                Some((key, val)) => (*key, (*val).clone()),
                None             => (0i64, Rc::new(None)),
            };
            */
            if *key <= revision {
                let res = match *val {
                    None => None,
                    Some(ref rct) => Some(rct.clone()),
                };
                return res; // TODO remove clone?
            }
        }
    }

    pub fn add_value(&mut self, revision: i64, value: Option<T>) {
        match value {
            None =>
                self.history.insert(revision, None),
            Some(real_val) =>
                self.history.insert(revision, Some(Rc::new(real_val)))
        };
    }
}

#[test]
fn element_init() {
    let mut node = VectorElement::<int>::new();
    assert_eq!(node.value(0i64), None);
    assert_eq!(node.value(1i64), None);
    node.add_value(1i64, Some(1807));
    assert_eq!(node.value(0i64), None);
    assert_eq!(*node.value(1i64).unwrap(), 1807);

    let mut node2 = VectorElement::<int>::new();
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
fn element_generic() {
    let mut node_str = VectorElement::<&str>::new();
    node_str.add_value(1, Some("hello"));
    node_str.add_value(10, Some("rust"));
    assert_eq!(node_str.value(0), None);
    assert_eq!(*node_str.value(1).unwrap(), "hello");
    assert_eq!(*node_str.value(2).unwrap(), "hello");
    assert_eq!(*node_str.value(9).unwrap(), "hello");
    assert_eq!(*node_str.value(10).unwrap(), "rust");
    assert_eq!(*node_str.value(11).unwrap(), "rust");

    let mut node_flt = VectorElement::<f32>::new();
    node_flt.add_value(12, Some(-1.0));
    node_flt.add_value(13, Some(1e-7));
    node_flt.add_value(14, Some(1.3333333));
    assert_eq!(node_flt.value(11), None); // TODO check with abs and eps
    assert!(abs_sub(*node_flt.value(12).unwrap(), -1.0) < 1e-8);
    assert!(abs_sub(*node_flt.value(13).unwrap(), 1e-7) < 1e-8);
    assert!(abs_sub(*node_flt.value(14).unwrap(), 1.3333333) < 1e-8);
    assert!(abs_sub(*node_flt.value(15).unwrap(), 1.3333333) < 1e-8);
}
