//! This file contains implementation of the linear congruential generator.
//! 
//! This generator guarantee that period is 2^31.
//! Parameters A, C, and M are taken from wiki article:
//! https://en.wikipedia.org/wiki/Linear_congruential_generator

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
