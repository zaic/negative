use std::rc::Rc;
use std::cell::RefCell;
use std::vec::Vec;
use std::fmt::Show;
use inner::persistent::*;
use inner::fat_field::*;
use inner::lcg_random::*;

type Link<A> = Option<Rc<RefCell<Node<A>>>>;

struct Node<A> {
    value: FatField<A>,
    prev:  FatField<Link<A>>,
    next:  FatField<Link<A>>
}

pub struct DList<A> {
    generator: Rc<RefCell<CoolLCG>>,
    tree:      Rc<RefCell<RevisionTree>>,
    front:     FatField<Link<A>>,
    back:      FatField<Link<A>>,
    head:      uint,
    history:   Vec<Revision>
}

pub struct DLIter<'a, A: 'a> {
    revision: Revision,
    link:     &'a Link<A>
}

impl<A> DList<A> {
    pub fn new() -> DList<A> {
        let generator: Rc<RefCell<CoolLCG>> = Rc::new(RefCell::new(LCG::new()));

        let root = generator.borrow_mut().next();
        let history = vec!(root);

        let head = 0;
        let tree = Rc::new(RefCell::new(RevisionTree::new(root)));

        let mut front = FatField::new(tree.clone());
        let mut back = FatField::new(tree.clone());

        front.insert(root, None);
        back.insert(root, None);

        DList {
            generator: generator,
            tree:      tree,
            front:     front,
            back:      back,
            head:      head,
            history:   history
        }
    }

    fn new_node(&self, r: Revision, v: A, p: Link<A>, n: Link<A>) -> Rc<RefCell<Node<A>>> {
        let mut vs = FatField::new(self.tree.clone());
        let mut ps = FatField::new(self.tree.clone());
        let mut ns = FatField::new(self.tree.clone());

        vs.insert(r, v);
        ps.insert(r, p);
        ns.insert(r, n);

        let n = Node {
            value: vs,
            prev:  ps,
            next:  ns
        };

        Rc::new(RefCell::new(n))
    }

    pub fn head(&self) -> Revision {
        self.history[self.head]
    }

    pub fn push(&mut self, v: A) {
        let h = self.head();
        let r = self.generator.borrow_mut().next();
        let f = match *self.front.get(h).unwrap() {
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

    pub fn push_back(&mut self, v: A) {
        let h = self.head();
        let r = self.generator.borrow_mut().next();
        let b = match *self.back.get(h).unwrap() {
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

    pub fn iter(&self, r: Revision) -> DLIter<A> {
        DLIter{revision: r, link: self.front.get(r).unwrap()}
    }
}

impl<A> Recall for DList<A> {
    fn undo_ntimes(&mut self, n: int) -> Revision {
        assert!(self.head as int - n as int >= 0);

        self.head -= n as uint;
        self.head()
    }

    fn redo_ntimes(&mut self, n: int) -> Revision {
        assert!(self.head + (n as uint) + 1u <= self.history.len());

        self.head += n as uint;
        self.head()
    }

    fn undo(&mut self) -> Revision {
        self.undo_ntimes(1)
    }

    fn redo(&mut self) -> Revision {
        self.undo_ntimes(1)
    }
}

impl<'a, A: Eq + Show> Iterator<&'a A> for DLIter<'a, A> {
    fn next(&mut self) -> Option<&'a A> {
        let r = self.revision;
        match *self.link {
            None        => None,
            Some(ref l) => {
                match l.borrow().next.get(r) {
                    None         => (),
                    Some(&ref n) => self.link = n
                };
                match l.borrow().value.get(r) {
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

#[cfg(test)]
fn assert<A: Show + Eq>(mut xs: DLIter<A>, es: &[A]) {
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
    xs.undo_ntimes(2);
    assert(xs.iter(xs.head()), &[1, 2]);
}

#[test]
fn undo_redo() {
    let mut xs: DList<int> = DList::new();
    xs.push_back(2);
    xs.push_back(1);
    xs.undo_ntimes(2);
    xs.redo_ntimes(1);
    xs.push_back(3);
    xs.push_back(4);

    assert(xs.iter(xs.head()), &[2, 3, 4]);
}
