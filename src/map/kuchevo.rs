#[allow(dead_code)]
use std::rc::Rc;

//#[deriving(Eq, Show)]
#[deriving(Show)]
pub enum Kuchevo {
    Nil,
    Node(int          /* key      */,
         int          /* priority */,
         Rc<Kuchevo>  /* left     */,
         Rc<Kuchevo>  /* right    */,)
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

impl Kuchevo {
    pub fn new_empty() -> Rc<Kuchevo> {
        Rc::new( Kuchevo::Node(0, 0, Rc::new(Kuchevo::Nil), Rc::new(Kuchevo::Nil)))
    }

    pub fn new(val: int, priority: int, left: Rc<Kuchevo>, right: Rc<Kuchevo>) -> Rc<Kuchevo> {
        Rc::new(Kuchevo::Node(val, priority, left, right))
    }

    pub fn merge(left: Rc<Kuchevo>, right: Rc<Kuchevo>) -> Rc<Kuchevo> {
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
            (&Kuchevo::Node(l_key, l_priortiy, ref l_child_left, ref l_child_right),
             &Kuchevo::Node(r_key, r_priortiy, ref r_child_left, ref r_child_right)) =>
                /*
                 *       L     >     R      =>        L
                 *      / \         / \     =>       / \
                 *     /   \       /   \    =>      /   \
                 *   L.L   L.R      ...     =>    L.L  merge(L.R, R)
                 */
                if l_priortiy > r_priortiy {
                    Rc::new(Kuchevo::Node(l_key,
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
                    Rc::new(Kuchevo::Node(r_key,
                                          r_priortiy,
                                          Kuchevo::merge(r_child_left.clone(),
                                                         left.clone()),
                                          r_child_right.clone()))

                }
        }
    }

    // return trees:
    // [-inf; mid) and [mid; +inf)
    pub fn split(&self, mid: int) -> (Rc<Kuchevo>, Rc<Kuchevo>) {
        match self {
            &Kuchevo::Nil =>
                (Rc::new(Kuchevo::Nil), Rc::new(Kuchevo::Nil)),

            &Kuchevo::Node(key, priority, ref left, ref right) =>
                if key < mid {
                    let (sp_left, sp_right) = 
                        if right.is_nil() {
                            (Rc::new(Kuchevo::Nil), Rc::new(Kuchevo::Nil))
                        } else {
                            right.split(key)
                        };

                    let res_left  = Rc::new(Kuchevo::Node(key,
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
                    let res_right = Rc::new(Kuchevo::Node(key,
                                                          priority,
                                                          sp_right,
                                                          right.clone()));
                    (res_left, res_right)
                }
        }
    }

    pub fn is_nil(&self) -> bool {
        match *self {
            Kuchevo::Nil => true,
            _            => false,
        }
    }
}

#[test]
fn kuchest() {
    let a = Kuchevo::new(0, 3, Kuchevo::new_empty(), Kuchevo::new_empty());
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

    let (l, m) = k.split(10); // Kuchevo::split(*k, 10);
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
