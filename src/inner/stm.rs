/*
 *  Initial STM implementation
 */

extern crate core;
use std::thread_local::OsStaticKey;
use std::thread_local::scoped::OS_INIT;
use self::core::mem;
pub use std::sync::RWLock;
pub use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

pub static TRANSACTION: OsStaticKey = OS_INIT;



pub struct Transaction<'a> {
    pub initial_time: int,
    pub failed: bool,
    pub log_read: HashMap<int, int>,
    pub log_write: HashMap<int, *const u8>, // TODO one more fix required...
    pub log_all: HashMap<int, Rc<RefCell<Box<TransactionCore + 'a>>>>
}

impl<'a> Transaction<'a> {
    pub fn new() -> Transaction<'a> {
        Transaction {
            initial_time: 0, // TODO use global clock
            failed: false,
            log_read: HashMap::new(),
            log_write: HashMap::new(),
            log_all: HashMap::new()
        }
    }

    pub fn commit(&mut self) {
        
    }
}

pub trait TransactionCore {
    fn check(&self) -> bool;
    fn commit(&mut self);
}

pub trait TransactionTrait<T> {
    //fn read<'a>(&'a self) -> &'a T;
    fn read(&self) -> T; // TODO use references with correct lifetime
    fn modify(&self, value: T);
}

pub struct Transactioned<T> {
    value: T,

    stm_clock: int,
    stm_uid: int,
    stm_guard: RWLock<()>,
}

impl<T> Transactioned<T> {
    pub fn new(init_val: T) -> Self {
        Transactioned {
            value: init_val,
            stm_clock: 0, // TODO
            stm_uid: 0, // TODO
            stm_guard: RWLock::new(())
        }
    }
    /*
     *  TODO (0): output lifetime is not static and (probably) will be fixed
     *  TODO (1): mark function as static?
     */
    pub fn get_transaction<'a>(&'a self) -> &'a mut Transaction {
        unsafe {
            let mut transaction_pointer : *mut Transaction = mem::transmute(TRANSACTION.get() as *mut Transaction);
            &mut *transaction_pointer
        }
    }
}

impl<T> TransactionCore for Transactioned<T> {
    fn check(&self) -> bool {
        let mut transaction = self.get_transaction();
        let current_struct_time = self.stm_clock;
        current_struct_time == transaction.initial_time
    }

    fn commit(&mut self) {
        let mut transaction = self.get_transaction();
        // TODO: get value from write_log, cast it and assign
    }
}

impl<'a, T: Clone> TransactionTrait<T> for Transactioned<T> {
    // 0. lock rwlock
    //
    // 1. verify global clock
    // 1a. fail transaction if clock was incremented (?)
    // 1b. insert clock value to read_log if it was'n present before
    //
    // 2. try to find self in write_log
    // 2a. use value from write_log
    // 2b. use self.value from self
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
                let a: *const u8 = transaction.log_write[self.stm_uid];
                let pa: *const u64 = a as *const u64;
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
        transaction.log_write.insert(self.stm_uid, unsafe { mem::transmute::<*const T, *const u8>(&value as *const T) }); // TODO remove transmute
    }
}
