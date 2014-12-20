use inner::kuchevo::Kuchevo;
use inner::lcg_random::*;
use inner::persistent::*;
use map::map_iterator::MapIterator;
use std::cell::RefCell;
use std::collections::tree_map::TreeMap;
use std::rc::Rc;
use std::vec::Vec;

pub type Node<K, V> = Rc<Kuchevo<K, V>>;
pub type SharedData<K, V> = Rc<RefCell<SharedMapData<K, V>>>;



pub struct SharedMapData<K, V> {
    pub last_revision:    Revision, // revision counter
    pub roots:            TreeMap<Revision, Node<K, V>>, // root tree node for each revision
    pub random:           CoolLCG, // random generator for priorities
}

pub struct PersMap<K, V> {
    line_history:        Vec<Revision>, // branch of history for undo-redo
    head_revision_id:    uint, // id of the current verision in line_history vector
    root:                Node<K, V>, // root node for current revision
    shared_data:         SharedData<K, V>, // pointer to above structure
}

impl<K: Ord + Clone, V: Clone> Persistent<PersMap<K, V>> for PersMap<K, V> {
    fn get_by_revision(&self, revision : Revision) -> PersMap<K, V> {
        assert!(revision <= self.shared_data.borrow().last_revision);
        assert!(self.shared_data.borrow().roots.contains_key(&revision));

        PersMap{line_history: vec![revision],
                head_revision_id: 0,
                root: self.shared_data.borrow().roots[revision].clone(),
                shared_data: self.shared_data.clone()}
    }

    fn current_revision_id(&self) -> Revision {
        assert!(self.line_history.len() > self.head_revision_id);

        self.line_history[self.head_revision_id]
    }
}

impl<K: Clone + Ord, V: Clone> Recall for PersMap<K, V> {
    fn undo(&mut self) -> Revision {
        assert!(self.head_revision_id > 0u);

        self.head_revision_id -= 1;
        self.root = self.head();
        self.line_history[self.head_revision_id]
    }

    fn redo(&mut self) -> Revision {
        assert!(self.head_revision_id + 1u < self.line_history.len());

        self.head_revision_id += 1;
        self.root = self.head();
        self.line_history[self.head_revision_id]
    }
}

impl<K: Clone + Ord, V: Clone> FullyPersistent<PersMap<K, V>> for PersMap<K, V> { }

impl<K: Ord + Clone, V: Clone> Clone for PersMap<K, V> {
    fn clone(&self) -> PersMap<K, V> { // TODO Self?
        PersMap{line_history: self.line_history.clone(),
                head_revision_id: self.head_revision_id,
                root: self.root.clone(),
                shared_data: self.shared_data.clone()}
    }
}
// TODO operator=

impl<K: Ord + Clone, V: Clone> PersMap<K, V> {
    pub fn new() -> PersMap<K, V> {
        let mut new_roots = TreeMap::new();
        new_roots.insert(1, Kuchevo::new_empty());
        let shdata = Rc::new(RefCell::new(SharedMapData::<K, V>{last_revision: 1,
                                                                roots: new_roots,
                                                                random: LCG::new()}));
        PersMap{line_history: vec![1],
                head_revision_id: 0,
                root: Kuchevo::new_empty(),
                shared_data: shdata}
    }

    fn head(&self) -> Node<K, V> {
        let rev = &self.line_history[self.head_revision_id];
        let root = self.shared_data.borrow().roots[*rev].clone();
        root
    }



    pub fn insert(&mut self, key: K, value: V) -> Revision {
        let old_root = self.root.clone(); //self.head();
        let mut data = self.shared_data.borrow_mut();
        let revision = data.last_revision + 1;

        let new_root = old_root.insert(Kuchevo::new_leaf(key, value, &data.random.next()));
        data.roots.insert(revision, new_root.clone());

        self.head_revision_id += 1;
        self.line_history.truncate(self.head_revision_id);
        self.line_history.push(revision);
        self.root = new_root;

        data.last_revision = revision;
        data.last_revision
    }

    pub fn remove(&mut self, key: &K) -> Revision {
        let old_root = self.root.clone(); //self.head();
        let mut data = self.shared_data.borrow_mut();
        let revision = data.last_revision + 1;

        let new_root = old_root.erase(key);
        data.roots.insert(revision, new_root.clone());

        self.head_revision_id += 1;
        self.line_history.truncate(self.head_revision_id);
        self.line_history.push(revision);
        self.root = new_root;

        data.last_revision += 1;
        data.last_revision
    }

    pub fn contains_key(&self, key: &K) -> bool {
        let mut node = self.root.clone(); //self.head();
        loop {
            let next_node = match node.deref() {
                &Kuchevo::Nil => return false,
                &Kuchevo::Node(ref nkey, ref _value, _priority, ref left, ref right) =>
                    if *nkey < *key {
                        right.clone()
                    } else if *nkey > *key {
                        left.clone()
                    } else {
                        return true;
                    }
            };
            node = next_node;
        }
    }

    pub fn iter<'a>(&'a self) -> MapIterator<'a, K, V> {
        MapIterator::new(&self.root)
    }
}

#[test]
fn map_insert_remove_test() {
    let mut m = PersMap::<int, ()>::new();
    println!("1");
    m.insert(10, ());
    println!("2");
    m.insert(20, ());
    println!("3");
    m.insert(30, ());
    println!("4");
    let map_before = m.current();
    m.remove(&30);
    let map_after  = m.current();
    m.remove(&25);
    m.remove(&20);

    assert_eq!(map_before.contains_key(&30), true);
    assert_eq!(map_after.contains_key(&30), false);
}

#[test]
fn map_iterator_test() {
    for q in range(2i, 100i) {
        let mut map = PersMap::<int, ()>::new();
        let max_key_val = q;

        for i in range(1i, max_key_val) {
            map.insert(i, ());
        }

        let mut expected_value = 1i;
        let cur_state = map.clone();
        println!("tree: {}", cur_state.root);
        for it in cur_state.iter() {
            println!("wow: {}", it);
            let (key, _) = it;
            assert_eq!(key, &expected_value);
            expected_value += 1;
        }
        assert_eq!(expected_value, q);
    }
}

#[test]
fn map_undoredo_test() {
    let mut map = PersMap::<int, &str>::new();

    map.insert(1, "one");
    map.insert(2, "two");
    println!("tree: {}", map.root);
    assert_eq!(map.contains_key(&2), true);
    assert_eq!(map.contains_key(&1), true);

    map.undo();
    println!("tree: {}", map.root);
    assert_eq!(map.contains_key(&2), false);
    assert_eq!(map.contains_key(&1), true);

    map.redo();
    println!("tree: {}", map.root);
    assert_eq!(map.contains_key(&2), true);
    assert_eq!(map.contains_key(&1), true);
}

#[test]
fn map_full_persistent_test() {
    let mut map = PersMap::<int, f32>::new();
    /*
     * My favourite tree:
     *
     *           7
     *          / 
     *         /
     *  1--2--4--6
     *      \
     *       \
     *        3--5--8
     */

    map.insert(2, 2.0);
    map.insert(3, 3.0);
    let mut three = map.clone();
    map.undo();
    map.insert(4, 4.0);
    three.insert(5, 5.0);
    let mut five = three.clone();
    let four_id = map.current_revision_id();
    map.insert(6, 6.0);
    let mut four = map.get_by_revision(four_id);
    four.insert(7, 7.0);
    five.insert(8, 8.0);

    assert!(five.contains_key(&3));
    assert!(five.contains_key(&5));
    assert!(five.contains_key(&8));
    assert!(!five.contains_key(&4));
    assert!(!five.contains_key(&7));

    assert!(four.contains_key(&7));
    assert!(!four.contains_key(&8));
    assert!(four.contains_key(&4));
    assert!(four.contains_key(&2));
    assert!(!four.contains_key(&3));

    assert!(five.get_by_revision(6).contains_key(&4));
    assert!(!five.get_by_revision(6).contains_key(&7));
}
