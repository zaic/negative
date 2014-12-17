use std::collections::tree_map::TreeMap;
use std::rc::Rc;
use inner::kuchevo::Kuchevo;
use inner::lcg_random::LCG;
use inner::lcg_random::CoolLCG;
use inner::persistent::Persistent;
use map::map_iterator::MapIterator;
use map::map_revision::MapRevision;



pub struct PersMap<K, V> {
    rev: i64,

    roots: TreeMap<i64, Rc<Kuchevo<K, V>>>,
    last_root: Rc<Kuchevo<K, V>>,

    prnd: CoolLCG,
}

impl<K: Ord + Clone, V: Clone> Persistent<MapRevision<K, V>> for PersMap<K, V> {
    fn get_by_revision(&self, revision : i64) -> MapRevision<K, V> {
        assert!(revision <= self.rev);
        MapRevision{rev: revision, root: self.roots[revision].clone()}
    }

    fn current_revision(&self) -> i64 {
        self.rev
    }

    fn undo(&mut self) -> i64 {
        panic!("Not implemented");
    }

    fn redo(&mut self) -> i64 {
        panic!("Not implemented");
    }
}

impl<K: Ord + Clone, V: Clone> PersMap<K, V> {
    pub fn new() -> PersMap<K, V> {
        PersMap{rev: 0, roots: TreeMap::new(), last_root: Rc::new(Kuchevo::Nil), prnd: LCG::new()}
    }



    pub fn insert(&mut self, key: K, value: V) -> i64 {
        self.rev += 1;
        self.last_root = (*self.last_root).insert(Kuchevo::new_leaf(key, value, &self.prnd.next()));
        self.roots.insert(self.rev, self.last_root.clone());
        self.rev
    }

    pub fn remove(&mut self, key: &K) -> i64 {
        self.rev += 1;
        self.last_root = self.last_root.erase(key);
        self.roots.insert(self.rev, self.last_root.clone());
        self.rev
    }
}

#[test]
fn map_insert_remove_test() {
    let mut m = PersMap::<int, ()>::new();
    m.insert(10, ());
    m.insert(20, ());
    m.insert(30, ());
    println!("root = {}", m.last_root);
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
