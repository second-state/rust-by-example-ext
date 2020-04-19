# Lazy static

The [`lazy_static`](https://docs.rs/lazy_static/1.4.0/lazy_static/) crate is a simple but widely used macro that allows you to initialize static variables at runtime.
To use it, add the following to your `Cargo.toml`.

```
[dependencies]
lazy_static = "1.4.0"
```

The example below shows three static variables that must be initialized
by code after the program starts.

```rust,editable
# extern crate lazy_static;

use std::collections::HashMap;

lazy_static! {
    static ref HASHMAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    };
    static ref COUNT: usize = HASHMAP.len();
    static ref NUMBER: u32 = times_two(21);
}

fn times_two(n: u32) -> u32 { n * 2 }

fn main() {
    println!("The map has {} entries.", *COUNT);
    println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());
    println!("A expensive calculation on a static results in: {}.", *NUMBER);
}
```

