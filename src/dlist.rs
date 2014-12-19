use std::rc::Rc;
use std::cell::RefCell as RCell;
use std::collections::HashMap as HMap;
use std::vec::Vec;
use std::fmt::Show;
use inner::versioned_fat_node::Revision;
use inner::versioned_fat_node::VersionTree as VTree;
use inner::lcg_random::LCG;
use inner::lcg_random::CoolLCG;

type Ref<A> = Rc<RCell<A>>;
type NRef<A> = Ref<Node<A>>;
type Link<A> = Option<NRef<A>>;

struct RMap<A> {
    values: HMap<Revision, A>,
    tree:   Ref<VTree>
}

struct Node<A> {
    value: RMap<A>,
    prev:  RMap<Link<A>>,
    next:  RMap<Link<A>>
}

pub struct DList<A: Clone> {
    generator: Ref<CoolLCG>,
    tree:      Ref<VTree>,
    front:     RMap<Link<A>>,
    back:      RMap<Link<A>>,
    head:      uint,
    history:   Vec<Revision>
}

pub struct DLIter<'a, A: 'a> {
    revision: Revision,
    link:     &'a Link<A>
}

fn new_ref<A>(a: A) -> Ref<A> {
    Rc::new(RCell::new(a))
}

impl<A: Clone> RMap<A> {
    pub fn new(tree: Ref<VTree>, r: Revision, v: A) -> RMap<A> {
        let mut values = HMap::new();
        values.insert(r, v);
        RMap {
            values: values,
            tree:   tree
        }
    }

    pub fn get(&self, r: Revision) -> &A {
        for e in self.tree.borrow().parent_branch(r).iter() {
            match self.values.get(e) {
                None         => continue,
                Some(&ref v) => return v
            }
        }
        panic!("we are doomed")
    }

    pub fn insert(&mut self, r: Revision, v: A) {
        self.values.insert(r, v);
    }
}

impl<A: Clone> DList<A> {
    pub fn new() -> DList<A> {
        let g: Ref<CoolLCG> = new_ref(LCG::new());

        let e = g.borrow_mut().next();
        let i = vec!(e);

        let h = 0;
        let t = new_ref(VTree::new(e));

        let f = RMap::new(t.clone(), e, None);
        let b = RMap::new(t.clone(), e, None);
        DList {
            generator: g,
            tree:      t,
            front:     f,
            back:      b,
            head:      h,
            history:   i
        }
    }

    fn new_node(&self, r: Revision, v: A, p: Link<A>, n: Link<A>) -> NRef<A> {
        new_ref(
            Node {
                value: RMap::new(self.tree.clone(), r, v),
                prev:  RMap::new(self.tree.clone(), r, p),
                next:  RMap::new(self.tree.clone(), r, n) 
            }
        )
    }

    pub fn head(&self) -> Revision {
        self.history[self.head]
    }

    pub fn undo(&mut self, n: uint) {
        let k = self.head as int - n as int;
        assert!(k >= 0);
        self.head -= n;
    }

    pub fn redo(&mut self, n: uint) {
        assert!(self.head + n + 1 <= self.history.len())
        self.head += n;
    }

    fn push(&mut self, v: A) {
        let h = self.head();
        let r = self.generator.borrow_mut().next();
        let f = match *self.front.get(h) {
            None => {
                let n = self.new_node(r, v, None, None);
                self.back.insert(r, Some(n.clone()));
                n
            },
            Some(ref f) => {
                let n = self.new_node(r, v, None, Some(f.clone()));
                f.borrow_mut().prev.insert(r, Some(n.clone()));
                n
            }
        };
        self.front.insert(r, Some(f.clone()));
        self.tree.borrow_mut().insert(r, h);
        self.head = self.history.len();
        self.history.push(r);
    }

    fn push_back(&mut self, v: A) {
        let h = self.head();
        let r = self.generator.borrow_mut().next();
        let b = match *self.back.get(h) {
            None => {
                let n = self.new_node(r, v, None, None);
                self.front.insert(r, Some(n.clone()));
                n
            },
            Some(ref b) => {
                let n = self.new_node(r, v, Some(b.clone()), None);
                b.borrow_mut().next.insert(r, Some(n.clone()));
                n
            }
        };
        self.back.insert(r, Some(b.clone()));
        self.tree.borrow_mut().insert(r, h);
        self.head = self.history.len();
        self.history.push(r);
    }

    pub fn push_array(&mut self, vs: &[A]) {
        for v in vs.iter() {
            self.push(v.clone());
        }
    }

    pub fn push_array_back(&mut self, vs: &[A]) {
        for v in vs.iter() {
            self.push_back(v.clone());
        }
    }

    pub fn iter(&self, r: Revision) -> DLIter<A> {
        DLIter{revision: r, link: self.front.get(r)}
    }
}

impl<'a, A: Clone + Eq + Show> Iterator<&'a A> for DLIter<'a, A> {
    fn next(&mut self) -> Option<&'a A> {
        let r = self.revision;
        match *self.link {
            None        => None,
            Some(ref l) => {
                /*
                self.link = l.borrow().next.get(r);
                Some(l.borrow().value.get(r))
                */
                match Some(l.borrow().next.get(r)) {
                    None         => (),
                    Some(&ref n) => self.link = n
                };
                match Some(l.borrow().value.get(r)) {
                    None         => None,
                    Some(&ref v) => Some(v)
                }
            }
        }
    }

    fn size_hint(&self) -> (uint, Option<uint>) {
        (0, None)
    }
}

macro_rules! dlist(
    ($($x:expr),*) => ({
        let mut x = DList::new();
        x.push_array_back(&[$($x),*]);
        x
    });
)

#[cfg(test)]
fn assert<A: Show + Clone + Eq>(mut xs: DLIter<A>, es: &[A]) {
    let mut i = 0;
    for x in xs {
        assert_eq!(*x, es[i]);
        i += 1;
    }
}

#[test]
fn push() {
    let mut xs: DList<int> = DList::new();
    xs.push(1);
    xs.push_back(2);
    xs.push(3);
    xs.push_back(4);

    assert(xs.iter(xs.head()), &[3, 1, 2, 4]);
    xs.undo(2);
    assert(xs.iter(xs.head()), &[1, 2]);
}

#[test]
fn undo_redo() {
    let mut xs: DList<int> = DList::new();
    xs.push_array_back(&[2, 1]);
    xs.undo(2);
    xs.redo(1);
    xs.push_array_back(&[3, 4]);

    assert(xs.iter(xs.head()), &[2, 3, 4]);
}
