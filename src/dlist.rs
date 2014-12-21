use std::rc::Rc;
use std::cell::*;
use std::fmt::Show;
use std::collections::HashMap;
use inner::persistent::*;
use inner::revision_tree::*;

type Link<A> = Option<Rc<RefCell<Node<A>>>>;

struct Node<A> {
    value: Field<A>,
    prev:  Field<Link<A>>,
    next:  Field<Link<A>>,
}

pub struct DList<A> {
    index: Rc<RefCell<uint>>,
    front: Rc<RefCell<Field<Link<A>>>>,
    back:  Rc<RefCell<Field<Link<A>>>>,
    tree:  Rc<RefCell<Tree>>
}

impl<A> DList<A> {
    pub fn new() -> DList<A> {
        let tree = Rc::new(RefCell::new(Tree::new()));
        let head = tree.borrow().root();

        let index = Rc::new(RefCell::new(0));
        let mut front = HashMap::new();
        let mut back = HashMap::new();

        front.insert(head, None);
        back.insert(head, None);

        DList {
            index: index,
            front: Rc::new(RefCell::new(front)),
            back:  Rc::new(RefCell::new(back)),
            tree:  tree
        }
    }

    fn new_node(&self, r: Revision, v: A, p: Link<A>, n: Link<A>) -> Rc<RefCell<Node<A>>> {
        let mut value = HashMap::new();
        let mut prev = HashMap::new();
        let mut next = HashMap::new();

        value.insert(r, v);
        prev.insert(r, p);
        next.insert(r, n);

        let node = Node {
            value: value,
            prev:  prev,
            next:  next
        };

        Rc::new(RefCell::new(node))
    }

    pub fn head(&self) -> Revision {
        self.tree.borrow().revision(*self.index.borrow())
    }

    pub fn push(&mut self, v: A) {
        let h = self.head();
        let r = self.tree.borrow_mut().fork(h);
        let f = match *self.tree.borrow()._get(self.front.borrow(), h).unwrap() {
            None => {
                let n = self.new_node(r, v, None, None);
                self.back.borrow_mut().insert(r, Some(n.clone()));
                n
            },
            Some(ref f) => {
                let n = self.new_node(r, v, None, Some(f.clone()));
                f.borrow_mut().prev.insert(r, Some(n.clone()));
                n
            }
        };
        self.front.borrow_mut().insert(r, Some(f.clone()));
        *self.index.borrow_mut() = self.tree.borrow().last_index();
    }

    pub fn push_back(&mut self, v: A) {
        let h = self.head();
        let r = self.tree.borrow_mut().fork(h);
        let b = match *self.tree.borrow()._get(self.back.borrow(), h).unwrap() {
            None => {
                let n = self.new_node(r, v, None, None);
                self.front.borrow_mut().insert(r, Some(n.clone()));
                n
            },
            Some(ref b) => {
                let n = self.new_node(r, v, Some(b.clone()), None);
                b.borrow_mut().next.insert(r, Some(n.clone()));
                n
            }
        };
        self.back.borrow_mut().insert(r, Some(b.clone()));
        *self.index.borrow_mut() = self.tree.borrow().last_index();
    }

    pub fn iter(&self, r: Revision) -> Items<A> {
        let link = self.tree.borrow()._get(self.front.borrow(), r).unwrap();
        Items {
            revision: r,
            link:     link,

            head:  Rc::new(RefCell::new(r)),
            index: self.index.clone(),
            tree:  self.tree.clone(),

            front: self.front.clone(),
            back:  self.back.clone()
        }
    }
}

impl<A> Recall for DList<A> {
    fn undo(&mut self) -> Revision {
        assert!(*self.index.borrow() > 0);

        *self.index.borrow_mut() -= 1;
        self.head()
    }

    fn redo(&mut self) -> Revision {
        assert!(*self.index.borrow() < self.tree.borrow().last_index());

        *self.index.borrow_mut() += 1;
        self.head()
    }
}

pub struct Items<'a, A: 'a> {
    revision: Revision,
    link:     &'a Link<A>,

    head:  Rc<RefCell<Revision>>,
    index: Rc<RefCell<uint>>,
    tree:  Rc<RefCell<Tree>>,

    front: Rc<RefCell<Field<Link<A>>>>,
    back:  Rc<RefCell<Field<Link<A>>>>
}

pub struct NodeRef<'a, A: 'a> {
    node:  Rc<RefCell<Node<A>>>,

    head:  Rc<RefCell<Revision>>,
    index: Rc<RefCell<uint>>,
    tree:  Rc<RefCell<Tree>>,

    front: Rc<RefCell<Field<Link<A>>>>,
    back:  Rc<RefCell<Field<Link<A>>>>
}

impl<'a, A: 'a> NodeRef<'a, A> {
    pub fn map(&self, f: |&'a A| -> A) {
        let h = *self.head.borrow();
        let r = self.tree.borrow_mut().fork(h);
        *self.head.borrow_mut() = r;
        *self.index.borrow_mut() = self.tree.borrow().last_index();
        let v = f(self.value());
        self.node.borrow_mut().value.insert(r, v);
    }

    pub fn remove(&self) {
        panic!("not yet implemented");
    }

    pub fn insert_before(&self) {
        panic!("not yet implemented");
    }

    pub fn insert_after(&self) {
        panic!("not yet implemented");
    }
    
    pub fn value(&self) -> &'a A {
        self.tree.borrow().get(&self.node.borrow().value, *self.head.borrow()).unwrap()
    }
}

impl<'a, A: 'a> Deref<A> for NodeRef<'a, A> {
    fn deref(&self) -> &A {
        self.value()
    }
}

impl<'a, A: Eq + Show> Iterator<NodeRef<'a, A>> for Items<'a, A> {
    fn next(&mut self) -> Option<NodeRef<'a, A>> {
        let revision = self.revision;
        match *self.link {
            None        => None,
            Some(ref link) => {
                self.link = self.tree.borrow().get(&link.borrow().next, revision).unwrap();
                let node_ref = NodeRef {
                    node:  link.clone(),

                    head:  self.head.clone(),
                    index: self.index.clone(),
                    tree:  self.tree.clone(),

                    front: self.front.clone(),
                    back:  self.back.clone()
                };
                Some(node_ref)
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
    xs.push(2);
    xs.push(1);
    let a = xs.head();

    xs.push_back(3);
    xs.push_back(4);
    let b = xs.head();

    assert(xs.iter(a), &[1, 2]);
    assert(xs.iter(b), &[1, 2, 3, 4]);
}

#[test]
fn undo_redo() {
    let mut xs: DList<int> = DList::new();
    xs.push(4);
    xs.push(3);
    xs.push(2);
    xs.push(1);
    let a = xs.head();

    let b = xs.undo_ntimes(2);
    let c = xs.redo_ntimes(2);

    assert(xs.iter(a), &[1, 2, 3, 4]);
    assert(xs.iter(b), &[3, 4]);
    assert(xs.iter(c), &[1, 2, 3, 4]);
}

#[test]
fn iter() {
    let mut xs: DList<int> = DList::new();

    xs.push(6);
    xs.push(5);
    xs.push(4);
    xs.push(3);
    let a = xs.head();

    xs.push(2);
    xs.push(1);
    let b = xs.head();

    for x in xs.iter(a) {
        x.map(|_| 0);
    }
    let c = xs.head();

    for x in xs.iter(a).filter(|x| **x > 4) {
        x.map(|_| 0);
    }
    let d = xs.head();

    assert(xs.iter(a), &[3, 4, 5, 6]);
    assert(xs.iter(b), &[1, 2, 3, 4, 5, 6]);
    assert(xs.iter(c), &[0, 0, 0, 0]);
    assert(xs.iter(d), &[3, 4, 0, 0]);
}
