#![feature(test)]

extern crate test;

use simple_rnd::Rand;

#[test]
fn random_test() {
    let mut rand = Rand::new(0);
    println!("{}", rand.next());
}
