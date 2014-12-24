//! This file contains implementation of the linear congruential generator.
//! 
//! This generator guarantee that period is 2^31.
//! Parameters A, C, and M are taken from wiki article:
//! https://en.wikipedia.org/wiki/Linear_congruential_generator

extern crate core;
use std::thread_local::OsStaticKey;
use std::thread_local::scoped::OS_INIT;
use self::core::mem;

use inner::stm::*;
use std::cell::RefCell;
use std::rc::Rc;

pub trait LCG {
    fn new() -> Self;
    fn next(&mut self) -> int;
}

pub struct CoolLCG {
    x: u64,
}

impl LCG for CoolLCG {
    fn new() -> CoolLCG {
        CoolLCG{x: 1807}
    }

    fn next(&mut self) -> int {
        let a = 1103515245u64;
        let c = 12345u64;
        let m = 0x80000000u64;
        self.x = (a * self.x + c) % m;
        self.x as int
    }
}

impl Copy for CoolLCG {}

#[test]
fn cool_lcg() {
    let mut rnd : CoolLCG = LCG::new();
    let one = rnd.next();
    let two = rnd.next();
    assert!(one != two);
}



pub struct DebugLCG {
    x: u64,
}

impl LCG for DebugLCG {
    fn new() -> DebugLCG {
        DebugLCG{x: 0}
    }

    fn next(&mut self) -> int {
        self.x += 1;
        self.x as int
    }
}

impl Copy for DebugLCG {}

#[test]
fn debug_lcg() {
    let mut rnd : DebugLCG = LCG::new();
    for i in range(1i, 10) {
        assert!(rnd.next() == i);
    }
}



pub struct TransactionLCG {
    x: Rc<RefCell<Box<Transactioned<u64>>>>
}

impl LCG for TransactionLCG {
    fn new() -> TransactionLCG {
        TransactionLCG { x: Rc::new(RefCell::new(box Transactioned::new(1807u64))) }
    }

    fn next(&mut self) -> int {
        // 0. lock rwlock
        //
        // 1. verify global clock
        // 1a. fail transaction if clock was incremented (?)
        // 1b. insert clock value to read_log if it was'n present before
        //
        // 2. try to find self in write_log
        // 2a. use self.x from write_log
        // 2b. use self.x from self
        //
        // 3. generate next value
        //
        // 4. store new x to write_log
        //
        // 5. unlock rwlock
        //
        //
        //
        // steps 0, 1[ ab] and 5 should be move out
        //
        // TLS is a thread-local map: stm_uid -> Something
        //   read_log is a map: stm_uid -> ClockType
        //   write_log is a map: stm_uid -> lamda (?)
        //   one more map: stm_uid -> &Transactioned

        let mut transaction = unsafe {
            let mut transaction_pointer : *mut Transaction = mem::transmute(TRANSACTION.get() as *mut Transaction);
            &mut *transaction_pointer
        };
        
        // 0
        let read_lock = self.stm_guard.read();

        // 1
        let current_struct_time = self.stm_clock;
        if current_struct_time != transaction.initial_time { // TODO use weak condition
            // TODO fail transaction
        } else {
            // it's ok
            //transaction.log_read[self.stm_uid] = current_struct_time; // TODO use global clock (?)
        }
        transaction.log_read.insert(self.stm_uid, current_struct_time); // TODO use global clock (?)

        // 2
        let x = if transaction.log_write.contains_key(&self.stm_uid) {
            transaction.log_write[self.stm_uid]
        } else {
            self.x
        };
        
        // 3
        let a = 1103515245u64;
        let c = 12345u64;
        let m = 0x80000000u64;
        let res = (a * x + c) % m;

        // 4
        transaction.log_write.insert(self.stm_uid, res);

        // 5
        // auto

        return res as int;
    }
}

#[test]
fn stm_random_test() {
    let mut cool_lcg: CoolLCG = LCG::new();
    let y0 = cool_lcg.next();
    let y1 = cool_lcg.next();
    println!("y0 = {} and y1 = {}", y0, y1);

    let mut lcg: TransactionLCG = LCG::new();
    {
        let mut transaction = Transaction::new();
        unsafe {
            let mut transaction_pointer: *mut u8 = mem::transmute(&mut transaction);
            TRANSACTION.set(transaction_pointer as *mut u8);
        }
        let x0 = lcg.next();
        let x1 = lcg.next();
        println!("x0 = {} and x1 = {}", x0, x1);
        assert_eq!(x0, y0);
        assert_eq!(x1, y1);
    }
    {
        let mut transaction = Transaction::new();
        unsafe {
            let mut transaction_pointer: *mut u8 = mem::transmute(&mut transaction);
            TRANSACTION.set(transaction_pointer as *mut u8);
        }
        let x0 = lcg.next();
        let x1 = lcg.next();
        println!("x0 = {} and x1 = {}", x0, x1);
        assert_eq!(x0, y0);
        assert_eq!(x1, y1);
        transaction.commit();
    }
    {
        let mut transaction = Transaction::new();
        unsafe {
            let mut transaction_pointer: *mut u8 = mem::transmute(&mut transaction);
            TRANSACTION.set(transaction_pointer as *mut u8);
        }
        let x0 = lcg.next();
        let x1 = lcg.next();
        println!("x0 = {} and x1 = {}", x0, x1);
        assert!(x0 != y0);
        assert!(x1 != y1);
    }
    //assert!(false);
}
