use inner::persistent::*;
use inner::versioned_fat_node::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::vec::Vec;

pub type SharedData<T> = Rc<RefCell<VectorSharedData<T>>>;



pub struct VectorSharedData<T> {
    last_revision: Revision,
    version_tree:  Rc<RefCell<VersionTree>>,

    ary:           Vec<VersionedFatNode<Option<Rc<T>>>>,
    len:           uint,
}

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



    pub fn len(&self) -> uint {
        self.ary.len()
    }

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
}

impl<T> Index<uint, T> for PersVector<T> {
    fn index<'a>(&'a self, id: &uint) -> &'a T {
        self.ary[*id].deref()
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
