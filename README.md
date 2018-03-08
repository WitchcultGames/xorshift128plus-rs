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
    //Create a new PRNG using a hard-coded seed
    let mut prng = XorShift128Plus::new(1337);

    //Create a new seed for the PRNG each time the application is run using system time
    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos() as u64;

    //Re-seed the PRNG using the time-based seed
    prng.seed(seed);

    //Perform the XorShift128Plus algorithm and return the next generated u64 number as is.
    //This method of number generation carries the least overhead, and all other functions are
    //convenience functions that does result in some more overhead, but it might be more efficient
    //for you to do this and then wrangle the result into the desired type yourself.
    //Allso, the correctness of the other functions is still questionable, at best...
    let next = prng.next();

    //Generate a pseudorandom i16 value
    let unsigned = prng.generate::<i16>();

    //Generate a pseudorandom f32 value between 0.0 and 1.0.
    //Usefull for scaling of other values.
    let factor = prng.random_factor();

    //Generate a pseudorandom bool based on on a chance input.
    //Usefull for picking the outcome of a random "dice roll" with a certain chance.
    //Input range is 0.0 - 1.0, where 0.0 guarantees faliure and 1.0 guarantees success.
    let chance = prng.chance(0.5);

    //Generate a pseudorandom i8 numbers in the range between -32 and 32;
    let signed_byte = prng.range(-32_i8, 32_i8);

    //Generate a pseudorandom f64 numbers in the range between -1234.56 and 6543.21;
    let double = prng.range(-1234.56_f64, 6543.21_f64);
}
```
