# Serialize third party library types

```rust,editable
# extern crate serde;
# extern crate num;

use num::bigint::ToBigInt;

fn main() {
    let a = 3.to_bigint().unwrap();
    let x = num::pow(a, 247);
    let xs = serde_json::to_string(&x).unwrap();
    println!("3**247 is {}", xs);
}
```
