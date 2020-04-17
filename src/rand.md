# Random numbers

A lot of applications require random numbers. The [`rand`](https://crates.io/crates/rand) crate is a very popular library in Rust to generate random numbers. 
It supports a wide variety of random number generators and distributions, each with a different performance and security trade off. 
For more details, you can read the [Rust Rand Book](https://rust-random.github.io/book/intro.html).
To use the `rand` crate, just do the following in your `Cargo.toml` file.

```
[dependencies]
rand = "0.7.3"
```

## Get a random number

To get a random number, you can simply do the following.

```rust,editable
use rand;

fn main() {
    let i: i32 = rand::random();
    println!("The random i32 is {}", i);
}
```

