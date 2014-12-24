//! This file contains implementation of the linear congruential generator.
//! 
//! This generator guarantee that period is 2^31.
//! Parameters A, C, and M are taken from wiki article:
//! https://en.wikipedia.org/wiki/Linear_congruential_generator

extern crate core;
use std::thread_local::OsStaticKey;
use std::thread_local::scoped::OS_INIT;
use core::mem;

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
    x: u64,

    stm_clock: int,
    stm_uid: int,
    stm_guard: RWLock<int>,
    stm_tls: Option<Rc<RefCell<TLS>>>,
}

impl Transactioned for TransactionLCG {
    fn check(&self) -> bool {
        // compare sefl.stm_clock and clock from read_log
        true
    }

    fn commit(&mut self, ld: |&Transactioned| -> int ) {
        // get self.x from write_log and store it
        // update stm_clock
    }

    fn unroll(&self) {
        // hmm... do nothing
    }
}

impl LCG for TransactionLCG {
    fn new() -> TransactionLCG {
        TransactionLCG {
            x: 1807,
            stm_clock: -1,
            stm_uid: 0, // TODO get global uid
            stm_guard: RWLock::new(0i),
            stm_tls: None}
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
        
        // 0
        let read_lock = self.stm_guard.read();
        let mut tls = self.stm_tls.unwrap().deref().borrow_mut();

        // 1
        let current_struct_time = self.stm_clock;
        if current_struct_time != tls.initial_time { // TODO use weak condition
            // TODO fail transaction
        } else {
            // it's ok
        }
        tls.log_read[self.stm_uid] = current_struct_time; // TODO use global clock (?)

        // 2
        let x = if tls.log_write.contains_key(&self.stm_uid) {
            tls.log_write[self.stm_uid]
        } else {
            self.x
        };
        
        // 3
        let a = 1103515245u64;
        let c = 12345u64;
        let m = 0x80000000u64;
        let res = (a * x + c) % m;

        // 4
        tls.log_write.insert(self.stm_uid, res);

        // 5
        // auto

        return res as int;
    }
}

#[test]
fn stm_random_test() {
    let mut testmap = BTreeMap::<int, int>::new();
    unsafe {
        let mut testp: *mut u8 = mem::transmute(&mut testmap);
        KEY.set(testp as *mut u8);
    }
    testmap.insert(10, 5);
    unsafe {
        let mut same_map_pointer : *mut BTreeMap<int, int> = mem::transmute(KEY.get() as *mut BTreeMap<int, int>);
        let mut same_map = &*same_map_pointer;
        println!("{}", same_map.len());
        println!("{}", same_map[10]);
    }
}
