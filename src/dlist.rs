use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::Show;
use inner::persistent::*;
use inner::fat_field::*;

type Link<A> = Option<Rc<RefCell<Node<A>>>>;

struct Node<A> {
    value: FatField<A>,
    prev:  FatField<Link<A>>,
    next:  FatField<Link<A>>
}

pub struct DList<A> {
    head_index: Rc<RefCell<uint>>,
    tree:       Rc<RefCell<RevisionTree>>,
    front:      FatField<Link<A>>,
    back:       FatField<Link<A>>
}

pub struct Items<'a, A: 'a> {
    revision: Revision,
    link:     &'a Link<A>
}

pub struct MutItems<'a, A: 'a> {
    revision:   Revision,
    head_index: Rc<RefCell<uint>>,
    link:       &'a Link<A>
}

impl<A> DList<A> {
    pub fn new() -> DList<A> {
        let tree = Rc::new(RefCell::new(RevisionTree::new()));
        let mut front = FatField::new(tree.clone());
        let mut back = FatField::new(tree.clone());
        let head_index = Rc::new(RefCell::new(0));
        let head = tree.borrow().root();

        front.insert(head, None);
        back.insert(head, None);

        DList {
            head_index: head_index,
            tree:       tree,
            front:      front,
            back:       back
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
        self.tree.borrow().history()[*self.head_index.borrow()]
    }

    pub fn push(&mut self, v: A) {
        let h = self.head();
        let r = self.tree.borrow_mut().fork(h);
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
        *self.head_index.borrow_mut() = self.tree.borrow().last_index();
    }

    pub fn push_back(&mut self, v: A) {
        let h = self.head();
        let r = self.tree.borrow_mut().fork(h);
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
        *self.head_index.borrow_mut() = self.tree.borrow().last_index();
    }

    pub fn iter(&self, r: Revision) -> Items<A> {
        Items {
            revision: r,
            link:     self.front.get(r).unwrap()
        }
    }

    pub fn iter_mut(&mut self, r: Revision) -> MutItems<A> {
        MutItems {
            revision:   r,
            head_index: self.head_index.clone(),
            link:       self.front.get(r).unwrap()
        }
    }
}

impl<A> Recall for DList<A> {
    fn undo_ntimes(&mut self, n: int) -> Revision {
        let h: int = *self.head_index.borrow() as int;
        assert!(h - n >= 0);

        *self.head_index.borrow_mut() -= n as uint;
        self.head()
    }

    fn redo_ntimes(&mut self, n: int) -> Revision {
        let h: int = *self.head_index.borrow() as int;
        assert!(h + n <= self.tree.borrow().last_index() as int);

        *self.head_index.borrow_mut() += n as uint;
        self.head()
    }

    fn undo(&mut self) -> Revision {
        self.undo_ntimes(1)
    }

    fn redo(&mut self) -> Revision {
        self.undo_ntimes(1)
    }
}

impl<'a, A: Eq + Show> Iterator<&'a A> for Items<'a, A> {
    fn next(&mut self) -> Option<&'a A> {
        let r = self.revision;
        match *self.link {
            None        => None,
            Some(ref l) => {
                self.link = l.borrow().next.get(r).unwrap();
                l.borrow().value.get(r)
            }
        }
    }

    fn size_hint(&self) -> (uint, Option<uint>) {
        (0, None)
    }
}

impl<'a, A: Eq + Show> Iterator<FatRef<'a, A>> for MutItems<'a, A> {
    fn next(&mut self) -> Option<FatRef<'a, A>> {
        let r = self.revision;
        match *self.link {
            None        => None,
            Some(ref l) => {
                self.link = l.borrow().next.get(r).unwrap();
                l.borrow().value.get_fat_ref(r, self.head_index.clone())
            }
        }
    }

    fn size_hint(&self) -> (uint, Option<uint>) {
        (0, None)
    }
}

#[cfg(test)]
fn assert<A: Show + Eq>(mut xs: Items<A>, es: &[A]) {
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

#[test]
fn iter_mut() {
    let mut xs: DList<int> = DList::new();
    xs.push_back(1);
    xs.push_back(2);
    xs.push_back(3);
    xs.push_back(4);

    let h = xs.head();
    for x in xs.iter_mut(h) {
        x.map(|x: &int| -> int {*x * *x});
    }

    assert(xs.iter(xs.head()), &[1, 4, 9, 16]);
}
