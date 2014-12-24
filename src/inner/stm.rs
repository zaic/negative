/*
 *  Initial STM implementation
 */

extern crate core;
use std::thread_local::OsStaticKey;
use std::thread_local::scoped::OS_INIT;
use self::core::mem;
pub use std::sync::RWLock;
pub use std::collections::HashMap;

pub static TRANSACTION: OsStaticKey = OS_INIT;



pub struct Transaction<'a> {
    pub initial_time: int,
    pub failed: bool,
    pub log_read: HashMap<int, int>,
    pub log_write: HashMap<int, u64>, // TODO one more fix required...
    pub log_all: HashMap<int, &'a TransactionCore>
}

impl Transaction {
    pub fn new() -> Transaction {
        Transaction {
            initial_time: 0, // TODO use global clock
            failed: false,
            log_read: HashMap::new(),
            log_write: HashMap::new()
        }
    }

    pub fn commit(&mut self) {
        
    }
}

pub trait TransactionCore {
    fn commit(&mut self);
}

pub trait TransactionTrait<T> {
    //fn read<'a>(&'a self) -> &'a T;
    fn read(&self) -> T; // TODO use references with correct lifetime
    fn modify(&self, value: T);

    fn check(&self) -> bool;
/*
    fn commit(&mut self, |&Transactioned| -> int); // быть может, вообще выпилить эту функцию
    */
}

pub struct Transactioned<T> {
    value: T,

    stm_clock: int,
    stm_uid: int,
    stm_guard: RWLock<()>,
}

impl<T> Transactioned<T> {
    pub fn get_transaction(&self) -> &'static mut Transaction { // TODO mark function as static?
        unsafe {
            let mut transaction_pointer : *mut Transaction = mem::transmute(TRANSACTION.get() as *mut Transaction);
            &mut *transaction_pointer
        }
    }
}

impl<T: Clone> TransactionTrait<T> for Transactioned<T> {
    /*
     *  0. get Transactino from TLS
     *  1. check time, do fail if variables was modified
     *  2. create entry in read log
     *  3. try to find value in write log
     *  4. return value
     */
    fn read(&self) -> T {
        let mut transaction = self.get_transaction();

        let current_struct_time = self.stm_clock;
        if current_struct_time != transaction.initial_time { // TODO use weak condition
            // TODO fail transaction
        } else {
            // it's ok
        }

        transaction.log_read.insert(self.stm_uid, current_struct_time); // TODO use global clock (?)
        
        let value = if transaction.log_write.contains_key(&self.stm_uid) {
            unsafe {
                let a = transaction.log_write[self.stm_uid];
                let pa: *const u64 = &a;
                let b = mem::transmute::<*const u64, *const T>(pa); 
                let c: T = (*b).clone();
                c.clone()
            } // TODO remove transmute
        } else {
            self.value.clone()
        };

        return value
    }

    /*
     *  0. get Transaction from TLS
     *  1. just create new udpate or update exists
     */
    fn modify(&self, value: T) {
        let mut transaction = self.get_transaction();
        transaction.log_write.insert(self.stm_uid, unsafe { *mem::transmute::<*const T, *const u64>(&value as *const T) }); // TODO remove transmute
    }

    fn check(&self) -> bool {
        true
    }
}
