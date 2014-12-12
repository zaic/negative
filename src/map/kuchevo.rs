use std::rc::Rc;
use std::default::Default;

#[deriving(Show)]
pub enum Kuchevo<T> {
    Nil,
    Node(T               /* key      */,
         int             /* priority */,
         Rc<Kuchevo<T>>  /* left     */,
         Rc<Kuchevo<T>>  /* right    */,)
}

/*
impl PartialEq for Kuchevo {
    fn eq(&self, other: &Kuchevo) -> bool {
        match (self, *other) {
            (&Kuchevo(Kuchevo::Nil), Kuchevo::Nil) => true,
            _                            => false,
        }
    }
}
*/

// TODO: remove Default
impl<T: Default + Ord + Clone> Kuchevo<T> {
    pub fn new_empty() -> Rc<Kuchevo<T>> {
        // TODO replacy by Kuchevo::Nil?
        Rc::new(Kuchevo::Node(Default::default(), 0, Rc::new(Kuchevo::Nil), Rc::new(Kuchevo::Nil)))
    }

    pub fn new_leaf(val: T, priority: &int) -> Rc<Kuchevo<T>> {
        Rc::new(Kuchevo::Node(val, *priority, Rc::new(Kuchevo::Nil), Rc::new(Kuchevo::Nil)))
    }

    pub fn new(val: T, priority: int, left: Rc<Kuchevo<T>>, right: Rc<Kuchevo<T>>) -> Rc<Kuchevo<T>> {
        Rc::new(Kuchevo::Node(val, priority, left.clone(), right.clone()))
    }

    pub fn is_nil(&self) -> bool {
        match *self {
            Kuchevo::Nil => true,
            _            => false,
        }
    }



    pub fn merge(left: Rc<Kuchevo<T>>, right: Rc<Kuchevo<T>>) -> Rc<Kuchevo<T>> {
        match (left.deref(), right.deref()) {
            // both nodes are Nil -- impossible
            (&Kuchevo::Nil, &Kuchevo::Nil) =>
                panic!("WTF?!"),

            // right is Nil, return left
            (&Kuchevo::Node(_, _, _, _), &Kuchevo::Nil) =>
                left.clone(),
                //Rc::new(Kuchevo::Node(l_key, l_priortiy, l_child_left, l_child_right)),

            // left is Nil, return right
            (&Kuchevo::Nil, &Kuchevo::Node(_, _, _, _)) =>
                right.clone(),
                //Rc::new(Kuchevo::Node(r_key, r_priortiy, r_child_left, r_child_right)),

            // merging
            (&Kuchevo::Node(ref l_key, l_priortiy, ref l_child_left, ref l_child_right),
             &Kuchevo::Node(ref r_key, r_priortiy, ref r_child_left, ref r_child_right)) =>
                /*
                 *       L     >     R      =>        L
                 *      / \         / \     =>       / \
                 *     /   \       /   \    =>      /   \
                 *   L.L   L.R      ...     =>    L.L  merge(L.R, R)
                 */
                if l_priortiy > r_priortiy {
                    Rc::new(Kuchevo::Node(l_key.clone(),
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
                                          r_priortiy,
                                          Kuchevo::merge(r_child_left.clone(),
                                                         left.clone()),
                                          r_child_right.clone()))

                }
        }
    }

    // return trees:
    // [-inf; mid) and [mid; +inf)
    pub fn split(&self, mid: &T) -> (Rc<Kuchevo<T>>, Rc<Kuchevo<T>>) {
        match self {
            &Kuchevo::Nil =>
                (Rc::new(Kuchevo::Nil), Rc::new(Kuchevo::Nil)),

            &Kuchevo::Node(ref key, priority, ref left, ref right) =>
                if *key < *mid {
                    let (sp_left, sp_right) = 
                        if right.is_nil() {
                            (Rc::new(Kuchevo::Nil), Rc::new(Kuchevo::Nil))
                        } else {
                            right.split(key)
                        };

                    let res_left  = Rc::new(Kuchevo::Node(key.clone(),
                                                          priority,
                                                          left.clone(),
                                                          sp_left));
                    let res_right = sp_right;
                    (res_left, res_right)

                } else {
                    let (sp_left, sp_right) = 
                        if left.is_nil() {
                            (Rc::new(Kuchevo::Nil), Rc::new(Kuchevo::Nil))
                        } else {
                            left.split(key)
                        };

                    let res_left  = sp_left;
                    let res_right = Rc::new(Kuchevo::Node(key.clone(),
                                                          priority,
                                                          sp_right,
                                                          right.clone()));
                    (res_left, res_right)
                }
        }
    }

    // return new root with new element
    pub fn insert(&self, value: Rc<Kuchevo<T>>) -> Rc<Kuchevo<T>> {
        let (key, priority) = match value.deref() {
            &Kuchevo::Nil              => panic!("wtf?"),
            &Kuchevo::Node(ref k, ref p, _, _) => (k.clone(), p),
        };
        let (less, greater) = self.split(&key);
        let value_kuchevo = Kuchevo::new_leaf(key, priority);
        Kuchevo::merge(less, Kuchevo::merge(value_kuchevo, greater))
    }

    // return new root without old element
    pub fn erase(&self, key: &T) -> Rc<Kuchevo<T>> {
        // TODO
        Kuchevo::new_empty()
    }
}

#[test]
fn compile_kuchest() {
    let a = Kuchevo::<int>::new(0, 3, Kuchevo::new_empty(), Kuchevo::new_empty());
    let b = Kuchevo::new(3, 3, Kuchevo::new_empty(), Kuchevo::new_empty());
    let c = Kuchevo::new(2, 4, a.clone(), b.clone());
    let ccc = Kuchevo::new(2, 4, a.clone(), b.clone());

    let d = Kuchevo::new(5, 1, Kuchevo::new_empty(), Kuchevo::new_empty());
    let e = Kuchevo::new(6, 2, d.clone(), Kuchevo::new_empty());
    let f = Kuchevo::new(4, 6, c.clone(), e.clone());

    let g = Kuchevo::new(11, 3, Kuchevo::new_empty(), Kuchevo::new_empty());
    let h = Kuchevo::new(9, 7, Kuchevo::new_empty(), g.clone());
    let i = Kuchevo::new(14, 4, Kuchevo::new_empty(), Kuchevo::new_empty());
    let j = Kuchevo::new(13, 8, h.clone(), i.clone());

    let k = Kuchevo::new(7, 10, f.clone(), j.clone());
    println!("{}", k);

    let (l, m) = k.split(&10);
    println!("{}", l);
    println!("{}", m);

    let n = Kuchevo::merge(m, l);
    println!("{}", n);

    let q = Kuchevo::new(10, 10, j, k);

/*
    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
    println!("{}", ccc);
    assert!(false);
    */
}

#[test]
fn insert_erase_kuchest() {
    let a = Kuchevo::<int>::new_leaf(0, &1);
    let b = a.insert(Kuchevo::new_leaf(10, &3));
    let c = b.insert(Kuchevo::new_leaf(20, &2));

    println!("{}", c);
    //assert!(false);
}
