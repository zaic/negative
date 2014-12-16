use std::collections::tree_map::TreeMap;
use std::rc::Rc;
use inner::kuchevo::Kuchevo;
use inner::lcg_random::LCG;
use inner::lcg_random::CoolLCG;
use map::map_entry::MapEntry;
use map::map_revision::MapRevision;



pub struct PersMap<K> {
    rev: i64,

    roots: TreeMap<i64, Rc<Kuchevo<K>>>,
    last_root: Rc<Kuchevo<K>>,

    prnd: CoolLCG,
}

impl<K: Ord + Clone> PersMap<K> {
    pub fn new() -> PersMap<K> {
        PersMap{rev: 0, roots: TreeMap::new(), last_root: Rc::new(Kuchevo::Nil), prnd: LCG::new()}
    }

    pub fn get_by_revision(&self, revision : i64) -> MapRevision<K> {
        assert!(revision <= self.rev);
        MapRevision{rev: revision, root: self.roots[revision].clone()}
    }

    pub fn current_revision(&self) -> i64 {
        self.rev
    }



    pub fn insert(&mut self, value: K) -> i64 {
        self.rev += 1;
        self.last_root = (*self.last_root).insert(Kuchevo::new_leaf(value, &self.prnd.next()));
        self.roots.insert(self.rev, self.last_root.clone());
        self.rev
    }

    pub fn remove(&mut self, value: &K) -> i64 {
        self.rev += 1;
        self.last_root = self.last_root.erase(value);
        self.roots.insert(self.rev, self.last_root.clone());
        self.rev
    }
}

#[test]
fn map_test() {
    let mut m = PersMap::<int>::new();
    m.insert(10);
    m.insert(20);
    m.insert(30);
    println!("root = {}", m.last_root);
    let map_before = m.get_by_revision(m.current_revision());
    m.remove(&30);
    let map_after  = m.get_by_revision(m.current_revision());
    m.remove(&25);
    m.remove(&20);

    assert_eq!(map_before.contains(&30), true);
    assert_eq!(map_after.contains(&30), false);
}
