# Serialize third party library types

Many third party Rust crates already support serialization and deserialization
via the `serde` crate. You just need to enable the `serde` feature when you 
declare the crate as dependency.

```
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
num-bigint = { version = "0.2", features = ["serde"] }
num = "0.2"
```

> **Note:** Since the `serde` feature is not enabled on third party crates in the Rust Playground by default, the code examples in this article are not interactive. We will show the program output in the text. You can run these examples from [this cargo project](https://github.com/second-state/rust-by-example-ext/tree/master/examples/serde/feature).

The example below shows how to serialize and deserialize a `BigInt` struct type from the `num_bigint` crate. It just works out of the box.

```rust,noplaypen
use num::bigint::ToBigInt;
use num_bigint::BigInt;

fn main() {
    let a = 3.to_bigint().unwrap();
    let x = num::pow(a, 247);
    let xs = serde_json::to_string(&x).unwrap();
    let xd: BigInt = serde_json::from_str(&xs).unwrap();
    assert_eq!(x, xd);

    println!("3**247 is {} and serializes to {}", x, xs);
}
```

Result

```
3**247 is 7062361041362837614435796717454722507454089864783271756927542774477268334591598635421519542453366332460075473278915787 and serializes to [1,[3323516107,3672165520,4080039719,3245710364,216105283,4292129601,4006727268,340573034,2851604588,3366124224,3797961372,1024846073,179]]
```

What about third party crates that are not yet to support the `serde` feature?
The `serde` crate provides another approach called [remote derive](https://serde.rs/remote-derive.html). Basically, you create a local copy of the remote type
and then serialize the local type through a bridge. Check out the [documentation](https://serde.rs/remote-derive.html).

