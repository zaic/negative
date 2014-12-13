/*
 *  This file contains implementation of the linear congruential generator.
 *
 *  This generator guarantee that period is 2^31.
 *  Parameters A, C, and M are taken from wiki article:
 *  https://en.wikipedia.org/wiki/Linear_congruential_generator
 */



pub struct LCG {
    x: u64,
}

impl LCG {
    pub fn new() -> LCG {
        LCG{x: 1807}
    }

    pub fn next(&mut self) -> int {
        let a = 1103515245u64;
        let c = 12345u64;
        let m = 0x80000000u64;
        self.x = (a * self.x + c) % m;
        self.x as int
    }
}

#[test]
fn lcg_random_test() {
    let mut rnd = LCG::new();
    let one = rnd.next();
    let two = rnd.next();
    assert!(one != two);
}
