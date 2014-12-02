use std::rc::Rc;

pub struct Kuchevo {
    x: int,
    priority: int,

    lchild: Option<Kuchevo>,
    rchild: Option<Kuchevo>,
}

impl Kuchevo {
    pub fn new() -> Kuchevo {
        Kuchevo{x: 0, priority: 0, lchild: None, rchild: None}
    }

    pub fn merge(oleft: &Option<Kuchevo>, oright: &Option<Kuchevo>) -> Kuchevo {
        assert!(oleft != None || oright != None);

        let left  = oleft.unwrap();
        let right = oright.unwrap();

        if oleft  == None { return right; }
        if oright == None { return left;  }

        assert!(left.priority != right.priority)
        if left.priority > right.priority {
            /*
             *       L     >     R      =>        L
             *      / \         / \     =>       / \
             *     /   \       /   \    =>      /   \
             *   L.L   L.R      ...     =>    L.L  merge(L.R, R)
             */
            Kuchevo{x: left.x,
                    priority: left.priority,
                    lchild: left.lchild,
                    rchild: Kuchevo::merge(left.rchild, oright)}
        } else {
            
        }

        Kuchevo::new()
    }

    pub fn split(&self, key: int) -> (Option<Kuchevo>, Option<Kuchevo>) {
        
    }
}

#[test]
fn kuchest() {
    let mut a = Kuchevo::new();

}
