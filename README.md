# Simple rnd

Tiny blazingly fast random number generation library.

###### examples/simple.rs
```rust
use simple_rnd::Rand;

fn main() {
    let mut rand = Rand::new(0); // The parameter is the starting value (seed)
    for _ in 0..16 {
        println!("{}", rand.next()); // rand.next() generates next u64 number
    }
}
```