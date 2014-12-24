/*
 *  Initial STM implementation
 */

pub use std::sync::RWLock;
pub use std::collections::HashMap;

pub struct TLS {
    pub initial_time: int,
    pub failed: bool,
    pub log_read: HashMap<int, int>,
    pub log_write: HashMap<int, u64>, // TODO one more fix required...
}

impl TLS {
    pub fn new() -> TLS {
        TLS {
            initial_time: 0, // TODO use global clock
            failed: false,
            log_read: HashMap::new(),
            log_write: HashMap::new(),
        }
    }
}

pub trait Transactioned {
    // these function like a gopher
    // fn read() -> Something;
    // fn modify();

    fn check(&self) -> bool;
    fn commit(&mut self, |&Transactioned| -> int); // быть может, вообще выпилить эту и следующую функцию
    fn unroll(&self);
}
