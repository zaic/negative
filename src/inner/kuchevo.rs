/*
 *  This file contains generic treap implementation.
 *
 *  Treap inner structure and basic treap operations described here:
 *      http://habrahabr.ru/post/101818/
 *
 *  Treap is a binary search tree, which allow O(lg(N)) inserting and removing.
 *  Treap can be easily converted to persistent data structure 
 *  using path-copying approach.
 */

use std::rc::Rc;
use std::fmt;

pub enum Kuchevo<K, V> {
    Nil,
    Node(K                  /* key      */,
         V                  /* value    */,
         int                /* priority */,
         Rc<Kuchevo<K, V>>  /* left     */,
         Rc<Kuchevo<K, V>>  /* right    */,)
}

impl<K: Ord + Clone, V: Clone> Kuchevo<K, V> {
    pub fn new_empty() -> Rc<Kuchevo<K, V>> {
        Rc::new(Kuchevo::Nil)
    }

    pub fn new_leaf(key: K, value: V, priority: &int) -> Rc<Kuchevo<K, V>> {
        Rc::new(Kuchevo::Node(key, value, *priority, Kuchevo::new_empty(), Kuchevo::new_empty()))
    }

    pub fn new(key: K, value: V, priority: int, left: Rc<Kuchevo<K, V>>, right: Rc<Kuchevo<K, V>>) -> Rc<Kuchevo<K, V>> {
        Rc::new(Kuchevo::Node(key, value, priority, left.clone(), right.clone()))
    }

    pub fn is_nil(&self) -> bool {
        match *self {
            Kuchevo::Nil => true,
            _            => false,
        }
    }



    pub fn merge(left: Rc<Kuchevo<K, V>>, right: Rc<Kuchevo<K, V>>) -> Rc<Kuchevo<K, V>> {
        match (left.deref(), right.deref()) {
            // both nodes are Nil -- impossible
            (&Kuchevo::Nil, &Kuchevo::Nil) =>
                panic!("WTF?!"),

            // right is Nil, return left
            (&Kuchevo::Node(_, _, _, _, _), &Kuchevo::Nil) =>
                left.clone(),

            // left is Nil, return right
            (&Kuchevo::Nil, &Kuchevo::Node(_, _, _, _, _)) =>
                right.clone(),

            // merging
            (&Kuchevo::Node(ref l_key, ref l_value, l_priortiy, ref l_child_left, ref l_child_right),
             &Kuchevo::Node(ref r_key, ref r_value, r_priortiy, ref r_child_left, ref r_child_right)) =>
                /*
                 *       L     >     R      =>        L
                 *      / \         / \     =>       / \
                 *     /   \       /   \    =>      /   \
                 *   L.L   L.R      ...     =>    L.L  merge(L.R, R)
                 */
                if l_priortiy > r_priortiy {
                    Rc::new(Kuchevo::Node(l_key.clone(),
                                          l_value.clone(),
                                          l_priortiy,
                                          l_child_left.clone(),
                                          Kuchevo::merge(l_child_right.clone(),
                                                         right.clone())))

                /*
                 *       L     <     R      =>             R
                 *      / \         / \     =>            / \
                 *     /   \       /   \    =>           /   \
                 *      ...      R.L   R.R  => merge(L, R.L) R.R
                 */
                } else {
                    Rc::new(Kuchevo::Node(r_key.clone(),
                                          r_value.clone(),
                                          r_priortiy,
                                          Kuchevo::merge(left.clone(),
                                                         r_child_left.clone()),
                                          r_child_right.clone()))

                }
        }
    }

    // return trees:
    // [-inf; mid), [mid; mid] and (mid; +inf)
    pub fn split(&self, mid: &K) -> (Rc<Kuchevo<K, V>>, Rc<Kuchevo<K, V>>, Rc<Kuchevo<K, V>>) {
        match self {
            &Kuchevo::Nil =>
                (Rc::new(Kuchevo::Nil), Rc::new(Kuchevo::Nil), Rc::new(Kuchevo::Nil)),

            &Kuchevo::Node(ref key, ref value, priority, ref left, ref right) =>
                if *key < *mid {
                    let (sp_left, sp_mid, sp_right) = 
                        if right.is_nil() {
                            (Rc::new(Kuchevo::Nil), Rc::new(Kuchevo::Nil), Rc::new(Kuchevo::Nil))
                        } else {
                            right.split(mid)
                        };

                    let res_left  = Rc::new(Kuchevo::Node(key.clone(),
                                                          value.clone(),
                                                          priority,
                                                          left.clone(),
                                                          sp_left));
                    let res_mid   = sp_mid;
                    let res_right = sp_right;
                    (res_left, res_mid, res_right)

                } else if *key > *mid {
                    let (sp_left, sp_mid, sp_right) = 
                        if left.is_nil() {
                            (Rc::new(Kuchevo::Nil), Rc::new(Kuchevo::Nil), Rc::new(Kuchevo::Nil))
                        } else {
                            left.split(mid)
                        };

                    let res_left  = sp_left;
                    let res_mid   = sp_mid;
                    let res_right = Rc::new(Kuchevo::Node(key.clone(),
                                                          value.clone(),
                                                          priority,
                                                          sp_right,
                                                          right.clone()));
                    (res_left, res_mid, res_right)
                } else {
                    let res_left  = left.clone();
                    let res_mid   = Rc::new(Kuchevo::Node(key.clone(),
                                                          value.clone(),
                                                          priority,
                                                          Rc::new(Kuchevo::Nil),
                                                          Rc::new(Kuchevo::Nil)));
                    let res_right = right.clone();
                    (res_left, res_mid, res_right)
                }
        }
    }

    // return new root with new element
    pub fn insert(&self, value: Rc<Kuchevo<K, V>>) -> Rc<Kuchevo<K, V>> {
        let (key, value, priority) = match value.deref() {
            &Kuchevo::Nil                             => panic!("wtf?"),
            &Kuchevo::Node(ref k, ref v, ref p, _, _) => (k.clone(), v.clone(), p),
        };
        let (less, _, greater) = self.split(&key);
        let value_kuchevo = Kuchevo::new_leaf(key, value, priority);
        Kuchevo::merge(less, Kuchevo::merge(value_kuchevo, greater))
    }

    // return new root without old element
    pub fn erase(&self, key: &K) -> Rc<Kuchevo<K, V>> {
        let (left, _, right) = self.split(key);
        Kuchevo::merge(left, right)
    }
}

impl<K: fmt::Show, V> fmt::Show for Kuchevo<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Kuchevo::Nil => 
                write!(f, "x"),

            &Kuchevo::Node(ref key, _, priority, ref left, ref right) =>
                write!(f, "(k={},p={},({},{}))", key, priority, **left, **right),
        }
    }
}



#[cfg(test)]
fn build_tree_from_habr() -> (Rc<Kuchevo<int, ()>>, String, String, String) {
    // build tree from reference article on habr
    // http://hsto.org/storage/habraeffect/a1/0a/a10a744def8f325a1019502ecc175ef6.png

    let a = Kuchevo::<int, ()>::new(0, (), 3, Kuchevo::new_empty(), Kuchevo::new_empty());
    let b = Kuchevo::new(3, (), 3, Kuchevo::new_empty(), Kuchevo::new_empty());
    let c = Kuchevo::new(2, (), 4, a.clone(), b.clone());
    let ccc = Kuchevo::new(2, (), 4, a.clone(), b.clone());
    let c_str = "(k=2,p=4,((k=0,p=3,(x,x)),(k=3,p=3,(x,x))))";
    assert_eq!(c_str, format!("{}", c).as_slice());

    let d = Kuchevo::new(5, (), 1, Kuchevo::new_empty(), Kuchevo::new_empty());
    let e = Kuchevo::new(6, (), 2, d.clone(), Kuchevo::new_empty());
    let f = Kuchevo::new(4, (), 6, c.clone(), e.clone());
    let f_str = format!("(k=4,p=6,({},(k=6,p=2,((k=5,p=1,(x,x)),x))))", c_str);
    assert_eq!(f_str, format!("{}", f));

    let g = Kuchevo::new(11, (), 3, Kuchevo::new_empty(), Kuchevo::new_empty());
    let h = Kuchevo::new(9, (), 7, Kuchevo::new_empty(), g.clone());
    let i = Kuchevo::new(14, (), 4, Kuchevo::new_empty(), Kuchevo::new_empty());
    let j = Kuchevo::new(13, (), 8, h.clone(), i.clone());
    let j_str = format!("(k=13,p=8,((k=9,p=7,(x,(k=11,p=3,(x,x)))),(k=14,p=4,(x,x))))");
    assert_eq!(j_str, format!("{}", j));

    let k = Kuchevo::new(7, (), 10, f.clone(), j.clone());
    let k_str = format!("(k=7,p=10,({},{}))", f_str, j_str);
    assert_eq!(k_str, format!("{}", k));

    (k, k_str, f_str, j_str)
}

#[test]
fn build_habr_kuchest() {
    build_tree_from_habr();
}

#[test]
fn split_to_three_kuchest() {
    let (root, _, left_tree_str, right_tree_str) = build_tree_from_habr();

    let (less, equal, greater) = root.split(&7);
    assert_eq!(left_tree_str, format!("{}", less));
    assert_eq!(right_tree_str, format!("{}", greater));
    assert_eq!("(k=7,p=10,(x,x))", format!("{}", equal).as_slice());
}

#[test]
fn split_and_merge_kuchest() {
    let (root, full_tree_str, left_tree_str, right_tree_str) = build_tree_from_habr();

    let (less, equal, greater) = root.split(&8);
    assert_eq!("x", format!("{}", equal).as_slice());
    let merged = Kuchevo::merge(less, greater);
    assert_eq!(full_tree_str, format!("{}", merged));
}

#[test]
fn build_habr_using_insert_kuchest() {
    let (_, full_tree_str, _, _) = build_tree_from_habr();

    let mut root = Kuchevo::<int, ()>::new_empty();
    let elements = [[0i, 3], [2, 4], [3, 3], [5, 1], [6, 2], [4, 6], [7, 10], [9, 7], [14, 4], [11, 3], [13, 8]];
    for i in elements.iter() {
        root = root.insert(Kuchevo::new_leaf(i[0], (), &i[1]));
    }
    assert_eq!(full_tree_str, format!("{}", root));
}

#[test]
fn insert_erase_kuchest() {
    let a = Kuchevo::<int, ()>::new_leaf(0, (), &1);
    let b = a.insert(Kuchevo::new_leaf(10, (), &3));
    let c = b.insert(Kuchevo::new_leaf(20, (), &2));
    assert_eq!("(k=10,p=3,((k=0,p=1,(x,x)),(k=20,p=2,(x,x))))",
               format!("{}", c).as_slice());

    let e = c.erase(&10);
    assert_eq!("(k=20,p=2,((k=0,p=1,(x,x)),x))",
               format!("{}", e).as_slice());
}

// TODO: large insert-erase tests
