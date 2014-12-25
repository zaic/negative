//! Persistent vector.
//!
//! Vector provides O(1) access to array element by indexes, push to the end and pop from the end.

use std::iter::repeat;
use inner::persistent::*;
use inner::versioned_fat_node::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::slice::Iter;
use std::vec::Vec;

pub type SharedData<T> = Rc<RefCell<VectorSharedData<T>>>;



pub struct VectorSharedData<T> {
    last_revision: Revision,
    version_tree:  Rc<RefCell<VersionTree>>,

    ary:           Vec<VersionedFatNode<Option<Rc<T>>>>,
    len:           uint,
}

/// Persistent vector implementation.
///
/// # Exmaples
///
/// Persistent vector supports all basic functions as ```std::Vec``` from the standart rust library.
///
/// ```
/// let mut vec: PersVector<int> = PersVector::new();
/// vec.push(1);
/// assert_eq!(vec[0], 1);
/// vec.pop();
/// assert_eq!(vec.len(), 0);
/// ```
///
/// But ```PersVector``` can give access to any previous revision:
///
/// ```
/// let mut v = PersVector::<int>::new();
/// v.push(1807);
/// let rev_before = v.push(2609);
/// v.pop();
/// v.push(1008);
///
/// let vec_before = v.get_by_revision(rev_before);
/// assert_eq!(vec_before[1], 2609);
/// ```
///
/// Undo-Redo is also supporting:
///
/// ```
/// let mut vector = PersVector::<int>::new();
///
/// vector.push(1807);
/// vector.push(2609);
/// assert_eq!(vector.len(), 2u);
///
/// vector.undo();
/// assert_eq!(vector.len(), 1u);
///
/// vector.redo();
/// assert_eq!(vector.len(), 2u);
/// assert_eq!(vector[0], 1807);
/// assert_eq!(vector[1], 2609);
/// ```
///
/// Moreover ```PersVector``` is fully persistent:
///
/// ```
/// let mut vector = PersVector::<int>::new();
/// 
/// vector.push(1807);
/// let rev_a = vector.push(2609);
/// vector.undo();
/// let rev_b = vector.push(1008);
/// 
/// let vector_a = vector.get_by_revision(rev_a);
/// assert_eq!(vector_a.len(), 2u);
/// assert_eq!(vector_a[0], 1807);
/// assert_eq!(vector_a[1], 2609);
/// 
/// let vector_b = vector.get_by_revision(rev_b);
/// assert_eq!(vector_b.len(), 2u);
/// assert_eq!(vector_b[0], 1807);
/// assert_eq!(vector_b[1], 1008);
/// ```
///
pub struct PersVector<T> {
    line_history:     Vec<Revision>, // branch of history for undo-redo
    head_revision_id: uint, // id of the current verision in line_history vector
    ary:              Vec<Rc<T>>, // array for the current revision

    shared_data:      SharedData<T>, // shared data between all revision
}

impl<T: Clone> Persistent<PersVector<T>> for PersVector<T> {
    fn get_by_revision(&self, revision : Revision) -> PersVector<T> {
        assert!(revision <= self.shared_data.deref().borrow().last_revision);

        let mut result_vector = Vec::<Rc<T>>::new();
        {
            let origin_ary = &self.shared_data.deref().borrow().ary;
            for it in origin_ary.iter() {
                match it.value(revision).unwrap_or(None) {
                    Some(ref rct) => result_vector.push(rct.clone()),
                    None          => break,
                };
            }
        }
        PersVector{line_history: vec![revision],
                   head_revision_id: 0,
                   ary: result_vector,
                   shared_data: self.shared_data.clone()}
    }

    fn current_revision_id(&self) -> Revision {
        self.line_history[self.head_revision_id]
    }
}

impl<T: Clone> Clone for PersVector<T> {
    fn clone(&self) -> Self {
        PersVector{line_history: self.line_history.clone(),
                   head_revision_id: self.head_revision_id,
                   ary: self.ary.clone(),
                   shared_data: self.shared_data.clone() }
    }
}

impl<T: Clone> Recall for PersVector<T> {
    fn undo(&mut self) -> Revision {
        assert!(self.head_revision_id > 0u);

        self.head_revision_id -= 1;
        let revision = self.line_history[self.head_revision_id];
        self.ary = self.get_by_revision(revision).ary;
        revision
    }   

    fn redo(&mut self) -> Revision {
        assert!(self.head_revision_id + 1u < self.line_history.len());

        self.head_revision_id += 1;
        let revision = self.line_history[self.head_revision_id];
        self.ary = self.get_by_revision(revision).ary;
        revision
    }
}

impl<T: Clone> FullyPersistent<PersVector<T>> for PersVector<T> { }

impl<T: Clone> PersVector<T> {
    /// Constructs a new, empty persistent vector.
    ///
    /// Created vector have one root revision.
    ///
    /// # Examples
    /// ```
    /// let mut pvec: PersVector<int> = PersVector::new();
    /// ```
    pub fn new() -> PersVector<T> {
        let vtree = Rc::new(RefCell::new(VersionTree::new(1)));
        let shdata = Rc::new(RefCell::new(VectorSharedData::<T>{last_revision: 1,
                                                                version_tree: vtree,
                                                                ary: Vec::new(),
                                                                len: 0}));
        PersVector{line_history: vec![1],
                   head_revision_id: 0,
                   ary: Vec::new(),
                   shared_data: shdata}
    }



    /// Returns the number of elements in the current vector revision.
    ///
    /// # Exmaples
    /// ```
    /// let mut vec = PersVector::<int>::new();
    /// vec.push(1);
    /// vec.push(2);
    /// assert_eq!(vec.len(), 2);
    /// ```
    pub fn len(&self) -> uint {
        self.ary.len()
    }

    /// Returns ```true``` if the vector contains no elements and false otherwise.
    ///
    /// # Exmaples
    /// ```
    /// let mut vec = PersVector::<int>::new();
    /// assert!(vec.empty());
    /// vec.push(1);
    /// vec.push(2);
    /// assert!(!vec.empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.ary.is_empty()
    }

    /// Append an element to the end of the vector.
    ///
    /// Returns new revision id.
    ///
    /// # Exmaples
    /// ```
    /// let mut vec = PersVector::<&str>::new();
    /// vec.push("pysch");
    /// ```
    pub fn push(&mut self, value: T) -> Revision {
        let mut shdata = self.shared_data.deref().borrow_mut();
        let old_rev = self.current_revision_id();
        let new_rev = shdata.last_revision + 1;

        // 1. update shared data
        shdata.last_revision = new_rev;
        let value_id = self.ary.len();
        if value_id == shdata.ary.len() {
            let vtree_pointer = shdata.version_tree.clone();
            shdata.ary.push(VersionedFatNode::new(vtree_pointer));
        }
        shdata.ary[value_id].add_value(new_rev, Some(Rc::new(value)), old_rev);
        shdata.len += 1;

        // 2. update local data
        self.head_revision_id += 1;
        self.line_history.truncate(self.head_revision_id);
        self.line_history.push(new_rev);
        self.ary.push(shdata.ary[value_id].value(new_rev).unwrap().unwrap());

        new_rev
    }

    /// Remove an element from the end of the vector.
    ///
    /// Returns new revision id.
    ///
    /// # Panics
    /// Panics if the vector is empty.
    ///
    /// # Exmaples
    /// ```
    /// let mut vec = PersVector::<&str>::new();
    /// vec.push("ololo");
    /// vec.pop();
    /// ```
    pub fn pop(&mut self) -> Revision {
        assert!(self.ary.len() > 0);
        let mut shdata = self.shared_data.deref().borrow_mut();
        let old_rev = self.current_revision_id();
        let new_rev = shdata.last_revision + 1;

        // 1. update shared data
        shdata.last_revision = new_rev;
        let value_id = self.ary.len() - 1;
        shdata.ary[value_id].add_value(new_rev, None, old_rev);
        shdata.len -= 1;

        // 2. update local data
        self.head_revision_id += 1;
        self.line_history.truncate(self.head_revision_id);
        self.line_history.push(new_rev);
        self.ary.pop();

        new_rev
    }

    /// Modify element in the vectory by it index.
    ///
    /// Returns new revision id.
    ///
    /// # Panics
    /// Panics if the index is out of bounds.
    ///
    /// # Exmaples
    /// ```
    /// let mut vec = PersVector::<&str>::new();
    /// vec.push("ololo");
    /// vec.modify(0, "pysch");
    /// ```
    pub fn modify(&mut self, id: uint, value: T) -> Revision {
        assert!(id < self.ary.len());
        let mut shdata = self.shared_data.deref().borrow_mut();
        let old_rev = self.current_revision_id();
        let new_rev = shdata.last_revision + 1;

        // 1. update shared data
        shdata.last_revision = new_rev;
        shdata.ary[id].add_value(new_rev, Some(Rc::new(value)), old_rev);

        // 2. update local data
        self.head_revision_id += 1;
        self.line_history.truncate(self.head_revision_id);
        self.line_history.push(new_rev);
        self.ary[id] = shdata.ary[id].value(new_rev).unwrap().unwrap();

        new_rev
    }

    /// Returns random-access iterator to the current revision vector.
    ///
    /// # Exmaples
    /// ```
    /// let mut vector = PersVector::<int>::new();
    /// ...
    /// for i in vector.iter() {
    ///     println!("one more vector element is {}", i.deref());
    /// }
    /// ```
    pub fn iter<'a>(&'a self) -> Iter<'a, Rc<T>> {
        self.ary.iter()
    }

    /// Shorten the vector, dropping excess elements.
    ///
    /// If ```len``` is greater than the vector's current length, this has no effect.
    ///
    /// # Exmaples
    /// ```
    /// let mut vec_ext = PersVector::<int>::new();
    /// vec_ext.push(1);
    /// vec_ext.push(2);
    /// assert_eq!(vec_ext.len(), 2);
    /// vec_ext.resize(5, 3);
    /// assert_eq!(vec_ext.len(), 5);
    /// vec_ext.truncate(3);
    /// assert_eq!(vec_ext.len(), 3);
    /// ```
    pub fn truncate(&mut self, len: uint) {
        while self.len() > len {
            self.pop();
        }
    }

    /// Resize the vector in-place.
    ///
    /// This function is equivalent to call ```truncate``` if the vector len is greater than
    /// ```new_len``` or to call ```extend``` if the vector len is less than ```new_len```.
    /// This function has no effect in case when ```new_len``` is equals to ```len()```.
    ///
    /// # Exmaples
    /// ```
    /// let mut vec_ext = PersVector::<int>::new();
    /// vec_ext.push(1);
    /// vec_ext.push(2);
    /// assert_eq!(vec_ext.len(), 2);
    /// ```
    pub fn resize(&mut self, new_len: uint, value: T) {
        self.truncate(new_len);
        let old_len = self.len();
        self.extend(repeat(value).take(new_len - old_len));
    }
}

impl<T: Clone> Index<uint, T> for PersVector<T> {
    fn index<'a>(&'a self, id: &uint) -> &'a T {
        self.ary[*id].deref()
    }
}

impl<T: Clone> Extend<T> for PersVector<T> {
    fn extend<I: Iterator<T>>(&mut self, mut iterator: I) {
        for element in iterator {
            self.push(element);
        }
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
    let pre_modify = v.current();
    v.modify(7, 1807);
    let post_modify = v.current();

    assert_eq!(pre_modify[7], 7);
    assert_eq!(post_modify[7], 1807);
}

#[test]
fn resize_test() {
    let mut vec_ext = PersVector::<int>::new();
    vec_ext.push(1);
    vec_ext.push(2);
    assert_eq!(vec_ext.len(), 2);
    vec_ext.resize(5, 3);
    assert_eq!(vec_ext.len(), 5);
    vec_ext.truncate(3);
    assert_eq!(vec_ext.len(), 3);
}

#[test]
fn vec_undoredo_test() {
    let mut vector = PersVector::<int>::new();

    vector.push(1807);
    vector.push(2609);
    assert_eq!(vector.len(), 2u);
    assert_eq!(vector[0], 1807);
    assert_eq!(vector[1], 2609);

    vector.undo();
    assert_eq!(vector.len(), 1u);
    assert_eq!(vector[0], 1807);

    vector.redo();
    assert_eq!(vector.len(), 2u);
    assert_eq!(vector[0], 1807);
    assert_eq!(vector[1], 2609);
}

#[test]
fn vec_fully_persistent_test() {
    let mut vector = PersVector::<int>::new();

    vector.push(1807);
    let rev_a = vector.push(2609);
    assert_eq!(vector.len(), 2u);
    assert_eq!(vector[0], 1807);
    assert_eq!(vector[1], 2609);

    vector.undo();
    assert_eq!(vector.len(), 1u);
    assert_eq!(vector[0], 1807);

    let rev_b = vector.push(1008);
    assert_eq!(vector.len(), 2u);
    assert_eq!(vector[0], 1807);
    assert_eq!(vector[1], 1008);

    let vector_a = vector.get_by_revision(rev_a);
    assert_eq!(vector_a.len(), 2u);
    assert_eq!(vector_a[0], 1807);
    assert_eq!(vector_a[1], 2609);

    let vector_b = vector.get_by_revision(rev_b);
    assert_eq!(vector_b.len(), 2u);
    assert_eq!(vector_b[0], 1807);
    assert_eq!(vector_b[1], 1008);
}

#[test]
fn vec_iterator_test() {
    let mut vector = PersVector::<int>::new();

    for i in range(1i, 10) {
        vector.push(i);
    }
    let mut expected_value = 1i;
    for i in vector.iter() {
        assert_eq!(i.deref(), &expected_value);
        expected_value += 1;
    }
    assert_eq!(expected_value, 10);
}
