//#[deriving(Eq, Show)]
#[deriving(Show)]
pub enum Kuchevo {
    Nil,
    Node(int          /* key      */,
         int          /* priority */,
         Box<Kuchevo> /* left     */,
         Box<Kuchevo> /* right    */,)
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
    pub fn new_empty() -> Box<Kuchevo> {
        box Kuchevo::Node(0, 0, box Kuchevo::Nil, box Kuchevo::Nil)
    }

    pub fn new(val: int, priority: int, left: Box<Kuchevo>, right: Box<Kuchevo>) -> Kuchevo {
        Kuchevo::Node(val, priority, left, right)
    }

    pub fn merge(left: Kuchevo, right: Kuchevo) -> Kuchevo {
        match (left, right) {
            // both nodes are Nil -- impossible
            (Kuchevo::Nil, Kuchevo::Nil) =>
                panic!("WTF?!"),

            // right is Nil, return left
            (Kuchevo::Node(l_key, l_priortiy, l_child_left, l_child_right), Kuchevo::Nil) =>
                Kuchevo::Node(l_key, l_priortiy, l_child_left, l_child_right),

            // left is Nil, return right
            (Kuchevo::Nil, Kuchevo::Node(r_key, r_priortiy, r_child_left, r_child_right)) =>
                Kuchevo::Node(r_key, r_priortiy, r_child_left, r_child_right),

            // merging
            (Kuchevo::Node(l_key, l_priortiy, l_child_left, l_child_right),
             Kuchevo::Node(r_key, r_priortiy, r_child_left, r_child_right)) =>
                /*
                 *       L     >     R      =>        L
                 *      / \         / \     =>       / \
                 *     /   \       /   \    =>      /   \
                 *   L.L   L.R      ...     =>    L.L  merge(L.R, R)
                 */
                if l_priortiy > r_priortiy {
                    Kuchevo::Node(l_key,
                                  l_priortiy,
                                  l_child_left,
                                  box Kuchevo::merge(*l_child_right,
                                                     Kuchevo::Node(r_key, r_priortiy, r_child_left, r_child_right)))

                /*
                 *       L     <     R      =>             R
                 *      / \         / \     =>            / \
                 *     /   \       /   \    =>           /   \
                 *      ...      R.L   R.R  => merge(L, R.L) R.R
                 */
                } else {
                    Kuchevo::Node(r_key,
                                  r_priortiy,
                                  box Kuchevo::merge(*r_child_left,
                                                     Kuchevo::Node(l_key, l_priortiy, l_child_left, l_child_right)),
                                  r_child_right)

                }
        }
    }

    // return trees:
    // [-inf; mid) and [mid; +inf)
    pub fn split(hren: Kuchevo, mid: int) -> (Kuchevo, Kuchevo) {
        match hren {
            Kuchevo::Nil =>
                (Kuchevo::Nil, Kuchevo::Nil),

            Kuchevo::Node(key, priority, left, right) =>
                if key < mid {
                    let (sp_left, sp_right) = 
                        if right.is_nil() {
                            (Kuchevo::Nil, Kuchevo::Nil)
                        } else {
                            //right.split(key)
                            Kuchevo::split(*right, key)
                        };

                    let res_left  = Kuchevo::Node(key,
                                                  priority,
                                                  left,
                                                  box sp_left);
                    let res_right = sp_right;
                    (res_left, res_right)

                } else {
                    let (sp_left, sp_right) = 
                        if left.is_nil() {
                            (Kuchevo::Nil, Kuchevo::Nil)
                        } else {
                            //left.split(key)
                            Kuchevo::split(*left, key)
                        };

                    let res_left  = sp_left;
                    let res_right = Kuchevo::Node(key,
                                                  priority,
                                                  box sp_right,
                                                  right);
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
    let a = box Kuchevo::new(0, 3, Kuchevo::new_empty(), Kuchevo::new_empty());
    let b = box Kuchevo::new(3, 3, Kuchevo::new_empty(), Kuchevo::new_empty());
    let c = box Kuchevo::new(2, 4, a, b);
    let d = box Kuchevo::new(5, 1, Kuchevo::new_empty(), Kuchevo::new_empty());
    let e = box Kuchevo::new(6, 2, d, Kuchevo::new_empty());
    let f = box Kuchevo::new(4, 6, c, e);

    let g = box Kuchevo::new(11, 3, Kuchevo::new_empty(), Kuchevo::new_empty());
    let h = box Kuchevo::new(9, 7, Kuchevo::new_empty(), g);
    let i = box Kuchevo::new(14, 4, Kuchevo::new_empty(), Kuchevo::new_empty());
    let j = box Kuchevo::new(13, 8, h, i);

    let k = box Kuchevo::new(7, 10, f, j);
    println!("{}", k);

    let (l, m) = Kuchevo::split(*k, 10);
    println!("{}", l);
    println!("{}", m);

    let n = box Kuchevo::merge(m, l);
    println!("{}", n);

    //assert!(false);
}
