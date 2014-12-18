use std::rc::Rc;
use std::cell::RefCell;
use std::collections::TreeMap as TMap;
use std::vec::Vec;
use std::fmt::Show;

pub type Revision = int;
type Link<A> = Option<NRef<A>>;

struct RMap<A>(TMap<Revision, A>);
type NRef<A> = Rc<RefCell<Node<A>>>;

struct Node<A> {
    value: RMap<A>,
    prev: RMap<Link<A>>,
    next: RMap<Link<A>>
}

struct RGen {
    revision: Revision
}

pub struct DList<A: Clone> {
    rgen: Rc<RefCell<RGen>>,
    revision: Vec<Revision>,
    size: RMap<uint>,
    front: RMap<Link<A>>,
    back: RMap<Link<A>>
}

pub struct DLIter<A> {
    revision: Revision,
    front: Link<A>,
    size: uint
}

impl RGen {
    fn new() -> RGen {
        RGen{revision: 0}
    }

    fn next(&mut self) -> Revision {
        self.revision += 1;
        self.revision
    }
}

impl<A> RMap<A> {
    fn new(r: Revision, v: A) -> RMap<A> {
        let mut m = TMap::new();
        m.insert(-r, v);
        RMap(m)
    }

    fn get(&self, r: Revision) -> &A {
        let &RMap(ref m) = self;

        match m.lower_bound(&(-r)).next() {
            None             => panic!("we are doomed"),
            Some((_, ref v)) => *v
        }
    }

    fn insert(&mut self, r: Revision, v: A) {
        let &RMap(ref mut m) = self;
        m.insert(-r, v);
    }
}

fn new_node<A>(r: Revision, v: A, p: Link<A>, n: Link<A>) -> NRef<A> {
    let v = RMap::new(r, v);
    let p = RMap::new(r, p);
    let n = RMap::new(r, n);
    Rc::new(RefCell::new(Node{value: v, prev: p, next: n}))
}

pub fn top<A>(v: &Vec<A>) -> &A {
    &v[v.len() - 1]
}

impl<A: Clone> DList<A> {
    pub fn new() -> DList<A> {
        let g = Rc::new(RefCell::new(RGen::new()));
        let r = vec!(0);
        let s = RMap::new(0, 0);
        let f = RMap::new(0, None);
        let b = RMap::new(0, None);
        DList{rgen: g, revision: r, size: s, front: f, back: b}
    }

    pub fn _last(&self) -> Revision {
        *top(&self.revision)
    }

    pub fn push_f(&mut self, v: A) {
        let r = self.rgen.borrow_mut().next();
        let f: NRef<A> = match self.front.get(r) {
            &None => {
                let n = new_node(r, v, None, None);
                self.back.insert(r, Some(n.clone()));
                n
            },
            &Some(ref f) => {
                let n = new_node(r, v, None, Some(f.clone()));
                f.borrow_mut().prev.insert(r, Some(n.clone()));
                n
            }
        };
        let &s = self.size.get(r);
        self.revision.push(r);
        self.front.insert(r, Some(f));
        self.size.insert(r, s + 1);
    }

    pub fn push_b(&mut self, v: A) {
        let r = self.rgen.borrow_mut().next();
        let b: NRef<A> = match self.back.get(r) {
            &None => {
                let n = new_node(r, v, None, None);
                self.front.insert(r, Some(n.clone()));
                n
            },
            &Some(ref b) => {
                let n = new_node(r, v, Some(b.clone()), None);
                b.borrow_mut().next.insert(r, Some(n.clone()));
                n
            }
        };
        let &s = self.size.get(r);
        self.revision.push(r);
        self.back.insert(r, Some(b));
        self.size.insert(r, s + 1);
    }

    pub fn push_nf(&mut self, vs: &[A]) {
        for v in vs.iter() {
            self.push_f(v.clone());
        }
    }

    pub fn push_nb(&mut self, vs: &[A]) {
        for v in vs.iter() {
            self.push_b(v.clone());
        }
    }

    pub fn iter(&self, r: Revision) -> DLIter<A> {
        let r = self.revision[r as uint];
        let f = match self.front.get(r) {
            &None        => None,
            &Some(ref n) => Some(n.clone())
        };
        let &s = self.size.get(r);

        DLIter{revision: r, front: f, size: s}
    }

    pub fn last(&self) -> DLIter<A> {
        self.iter((self.revision.len() - 1) as Revision)
    }
}

pub fn cat<A: Clone>(l_1: &mut DList<A>, l_2: &mut DList<A>) {
    let r_1 = l_1._last();
    let r_2 = l_2._last();
    let &s_1 = l_1.size.get(r_1);
    let &s_2 = l_2.size.get(r_2);

    if r_1 > r_2 {
        l_2.rgen = l_1.rgen.clone();
    }
    else {
        l_1.rgen = l_2.rgen.clone();
    }

    let r = l_1.rgen.borrow_mut().next();
    let f = match l_1.front.get(r_1) {
        &None         => None,
        &Some (ref f) => Some(f.clone())
    };
    let b = match l_2.back.get(r_2) {
        &None         => None,
        &Some (ref b) => Some(b.clone())
    };
    let s = s_1 + s_2;

    l_1.revision.push(r);
    l_2.revision.push(r);
    l_1.back.insert(r, b);
    l_2.front.insert(r, f);
    l_1.size.insert(r, s);
    l_2.size.insert(r, s);

    match l_1.back.get(r_1) {
        &None => {},
        &Some(ref b) => {
            match l_2.front.get(r_2) {
                &None => {},
                &Some (ref f) => {
                    b.borrow_mut().next.insert(r, Some(f.clone()));
                    f.borrow_mut().prev.insert(r, Some(b.clone()));
                }
            }
        }
    };
}

impl<A: Clone + Eq + Show> Iterator<A> for DLIter<A> {
    fn next(&mut self) -> Option<A> {
        if self.size == 0 {
            None
        }
        else {
            let r = self.revision;
            let (v, f) = match self.front {
                None         => (None, None),
                Some(ref _f) => {
                    let f = _f.borrow();

                    let &ref v = f.value.get(r);
                    let n = match f.next.get(r) {
                        &None        => None,
                        &Some(ref s) => Some(s.clone())
                    };

                    (Some(v.clone()), n)
                }
            };
            self.front = f;
            self.size -= 1;
            v
        }
    }

    fn size_hint(&self) -> (uint, Option<uint>) {
        (0, None)
    }
}

macro_rules! dlist(
    ($($x:expr),*) => ({
        let mut x = DList::new();
        x.push_nb(&[$($x),*]);
        x
    });
)

fn assert<A: Show + Clone + Eq>(mut xs: DLIter<A>, es: &[A]) {
    let mut i = 0;
    for x in xs {
        assert_eq!(x, es[i]);
        i += 1;
    }
}

#[test]
fn push() {
    let mut xs: DList<int> = DList::new();
    xs.push_f(1);
    xs.push_b(2);
    xs.push_f(3);
    xs.push_b(4);

    assert(xs.iter(2), &[1, 2]);
    assert(xs.iter(4), &[3, 1, 2, 4]);
}

#[test]
fn cat_() {
    let mut xs = dlist!(3i, 4, 5);
    let mut ys = dlist!(6i, 7);

    cat(&mut xs, &mut ys);
    ys.push_nb(&[8, 9]);
    xs.push_nf(&[2, 1]);

    assert(xs.last(), &[1, 2, 3, 4, 5, 6, 7]);
    assert(ys.last(), &[3, 4, 5, 6, 7, 8, 9]);
}
