#![feature(test)]

extern crate test;
use test::Bencher;

use simple_rnd::Rand;

#[bench]
fn random_bench(b: &mut Bencher) {
    let mut rand = Rand::new(0);
    b.iter(|| rand.next());
}
