/*
 *  Initial STM implementation
 */

use std::collections::BTreeMap as TreeMap;
use std::rc::Rc;
use inner::persistent::Revision;

pub trait Transactioned {
    // these function like a gopher
    // fn read() -> Something;
    // fn modify();

    fn check(&self) -> bool;
    fn commit(&mut self, lambda); // быть может, вообще выпилить эту и следующую функцию
    fn unroll(&self);
}
