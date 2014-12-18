use std::collections::tree_map::TreeMap;
use std::rc::Rc;
use std::vec::Vec;
use inner::kuchevo::Kuchevo;
use inner::lcg_random::LCG;
use inner::lcg_random::CoolLCG;
use inner::persistent::Persistent;
use map::map_iterator::MapIterator;
use map::map_revision::MapRevision;

type Node<K, V> = Rc<Kuchevo<K, V>>;



pub struct PersMap<K, V> {
    line_history: Vec<i64>, // branch of history for undo-redo
    head_revision_id: uint,  // id of the current verision in line_history vector
    last_revision: i64,     // revisions counter

    roots: TreeMap<i64, Rc<Kuchevo<K, V>>>, // root tree node for each revision

    prnd: CoolLCG, // random generator for priorities
}

impl<K: Ord + Clone, V: Clone> Persistent<MapRevision<K, V>> for PersMap<K, V> {
    fn get_by_revision(&self, revision : i64) -> MapRevision<K, V> {
        assert!(revision <= self.last_revision);
        assert!(self.roots.contains_key(&revision));

        MapRevision{rev: revision, root: self.roots[revision].clone()}
    }

    fn current_revision(&self) -> i64 {
        assert!(self.line_history.len() > self.head_revision_id);

        self.line_history[self.head_revision_id]
    }

    fn undo(&mut self) -> i64 {
        assert!(self.head_revision_id > 0u);

        self.head_revision_id -= 1;
        self.line_history[self.head_revision_id]
    }

    fn redo(&mut self) -> i64 {
        assert!(self.head_revision_id < self.line_history.len() - 1u);

        self.head_revision_id += 1;
        self.line_history[self.head_revision_id]
    }
}

impl<K: Ord + Clone, V: Clone> PersMap<K, V> {
    pub fn new() -> PersMap<K, V> {
        let mut new_roots = TreeMap::new();
        new_roots.insert(1, Kuchevo::new_empty());
        PersMap{line_history: vec![1],
                head_revision_id: 0,
                last_revision: 1,
                roots: new_roots,
                prnd: LCG::new()}
    }



    fn head(&self) -> Node<K, V> {
        let a = self.head_revision_id;
        let b: i64 = self.line_history[a].clone();
        let c = self.roots[b].clone();
        c
    }

    pub fn insert(&mut self, key: K, value: V) -> i64 {
        self.last_revision += 1;

        let old_root = self.head();
        let new_root = old_root.insert(Kuchevo::new_leaf(key, value, &self.prnd.next()));
        self.roots.insert(self.last_revision, new_root);

        self.head_revision_id += 1;
        self.line_history.truncate(self.head_revision_id);
        self.line_history.push(self.last_revision);

        self.last_revision
    }

    pub fn remove(&mut self, key: &K) -> i64 {
        self.last_revision += 1;

        let old_root = self.head();
        let new_root = old_root.erase(key);
        self.roots.insert(self.last_revision, new_root);

        self.head_revision_id += 1;
        self.line_history.truncate(self.head_revision_id);
        self.line_history.push(self.last_revision);

        self.last_revision
    }
}

#[test]
fn map_insert_remove_test() {
    let mut m = PersMap::<int, ()>::new();
    m.insert(10, ());
    m.insert(20, ());
    m.insert(30, ());
    //println!("root = {}", m.last_root);
    let map_before = m.last_revision();
    m.remove(&30);
    let map_after  = m.last_revision();
    m.remove(&25);
    m.remove(&20);

    assert_eq!(map_before.contains(&30), true);
    assert_eq!(map_after.contains(&30), false);
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
        let cur_state = map.last_revision();
        println!("tree: {}", cur_state.root);
        for it in cur_state.iter() {
            println!("wow: {}", it);
            let (a, b) = it;
            assert_eq!(a, &expected_value);
            expected_value += 1;
        }
    }
}

#[test]
fn map_undoredo_test() {
    let mut map = PersMap::<int, &str>::new();

    map.insert(1, "one");
    map.insert(2, "two");
    assert_eq!(map.last_revision().contains(&2), true);
    assert_eq!(map.last_revision().contains(&1), true);

    map.undo();
    assert_eq!(map.last_revision().contains(&2), false);
    assert_eq!(map.last_revision().contains(&1), true);

    map.redo();
    assert_eq!(map.last_revision().contains(&2), true);
    assert_eq!(map.last_revision().contains(&1), true);
}
