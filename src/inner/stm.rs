/*
 *  Initial STM implementation
 */

pub use std::sync::RWLock;
pub use std::collections::HashMap;

pub struct Transaction {
    pub initial_time: int,
    pub failed: bool,
    pub log_read: HashMap<int, int>,
    pub log_write: HashMap<int, u64>, // TODO one more fix required...
}

impl Transaction {
    pub fn new() -> Transaction {
        Transaction {
            initial_time: 0, // TODO use global clock
            failed: false,
            log_read: HashMap::new(),
            log_write: HashMap::new(),
        }
    }
}

pub trait TransactionTrait<T> {
    fn read<'a>(&'a self) -> &'a T;
    fn modify(value: &T);

    fn check(&self) -> bool;
/*
    fn commit(&mut self, |&Transactioned| -> int); // быть может, вообще выпилить эту и следующую функцию
    fn unroll(&self);
    */
}
