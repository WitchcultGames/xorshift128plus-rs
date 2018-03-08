# xorshift128plus-rs
A simple implementation of the XorShift128+ pseudorandom number generator in Rust.

## Simple usage example:
#### ./Cargo.toml
```toml
[dependencies]
xorshift128plus-rs = "0.1.3"
```
#### ./src/main.rs
```rust
extern crate xorshift128plus_rs;

use std::time::{UNIX_EPOCH, SystemTime};
use xorshift128plus_rs::XorShift128Plus;

fn main() {
    //Create a new seed for the PRNG each time the application is run using system time
    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos() as u64;

    //Create a new PRNG using the seed
    let mut prng = XorShift128Plus::new(seed);

    //Print 100 random i8 numbers in the range between -32 and 32 to stdout
    for _ in 0..100 {
        println!("{}", prng.range(-32_i8, 32_i8));
    }
}
```
