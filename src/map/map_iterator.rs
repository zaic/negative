use std::collections::tree_map::TreeMap;
use std::fmt;
use std::rc::Rc;
use std::vec::Vec;
use inner::kuchevo::Kuchevo;



pub struct MapIterator<'a, K: 'a, V: 'a> {
    node: &'a Rc<Kuchevo<K, V>>,
    branch: Vec<(&'a Rc<Kuchevo<K, V>>, u8)>,
    direction: u8,
}

impl<'a, K: 'a, V: 'a> MapIterator<'a, K, V> {
    pub fn new(root_node: &'a Rc<Kuchevo<K, V>>) -> MapIterator<'a, K, V> {
        MapIterator{node: root_node,
                    branch: Vec::new(),
                    direction: 0}
    }
}

impl<'a, K: Clone + Ord + fmt::Show + 'a, V: Clone + fmt::Show + 'a> Iterator<(&'a K, &'a V)> for MapIterator<'a, K, V> {
    fn next(&mut self) -> Option<(&'a K, &'a V)> {
        loop {
            let (res, left, right) = match self.node.deref() {
                &Kuchevo::Nil => return None,
                &Kuchevo::Node(ref key, ref value, _, ref left, ref right) => (Some((key, value)), left, right),
            };
            //println!("res = {}, left = {}, right = {}, go_right = {}", res, left, right, self.direction);

            if self.direction == 0 && !left.is_nil() { // go to the left
                self.branch.push((self.node, 1));
                self.node = left;
                self.direction = 0;
                
            } else if self.direction != 2 && !right.is_nil() { // go to the right
                self.branch.push((self.node, 2));
                self.node = right;
                self.direction = 0;
                return res;

            } else if !self.branch.is_empty() { // go to the up
                let (a, b) = self.branch.pop().unwrap();
                self.node = a;
                if self.direction != 2 {
                    self.direction = b;
                    return res;
                } else {
                    self.direction = b;
                }

            } else {
                return if self.direction < 2 {
                    self.direction = 3;
                    res
                } else {
                    None
                };
            }
        }
    }
}
