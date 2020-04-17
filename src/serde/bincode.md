# Serialize into binary

JSON strings are portable across almost all programming languages and frameworks. But for communication between Rust programs, a binary format could be much more efficient. Here is where `bincode` comes into play. To use the `bincode` crate, you just need to add the following dependencies to your `Cargo.toml` file.

```
[dependencies]
serde = { version = "1.0", features = ["derive"] }
bincode = "1.2.1"
```

Note: Since the `bincode` crate is not included the Rust Playground by default, the code examples in this article are not interactive. We will show the program output in the text. You can run these examples from [this cargo project](https://github.com/second-state/rust-by-example-ext/tree/master/examples/serde/bincode).

The example below shows how to serialize a simple Rust primitive data type `i32` into a byte array, and then deserialize it back. You can pass this byte array to other Rust programs over the Internet.

```rust,noplaypen
use bincode;

fn main() {
    let x: i32 = 5;
    let xs: Vec<u8> = bincode::serialize(&x).unwrap();
    println!("i32 number {} serializes into byte array {:?}", x, xs);
    let xd: i32 = bincode::deserialize(&xs).unwrap();
    assert_eq!(x, xd);
}
```

Result

```
i32 number 5 serializes into byte array [5, 0, 0, 0]
```

Here are more examples showing the serialization and deserialization of Rust primitive data types. Edit the code below to try more Rust types, and run it to see the results.

```rust,noplaypen
use bincode;

fn main() {
    let x: i32 = 5;
    let xs: Vec<u8> = bincode::serialize(&x).unwrap();
    println!("i32 number {} serializes into byte array {:?}", x, xs);
    let xd: i32 = bincode::deserialize(&xs).unwrap();
    assert_eq!(x, xd);

    let x: f32 = 3.14;
    let xs = bincode::serialize(&x).unwrap();
    println!("f32 number {} serializes into byte array {:?}", x, xs);

    let x: Vec<u8> = vec![1, 2, 3];
    let xs = bincode::serialize(&x).unwrap();
    println!("Vec<u8> {:?} serializes into byte array {:?}", x, xs);
    let xd: Vec<u8> = bincode::deserialize(&xs).unwrap();
    assert_eq!(x, xd);

    let x: Vec<f32> = vec![3.141, 2.718, 1.618];
    let xs = bincode::serialize(&x).unwrap();
    println!("Vec<f32> {:?} serializes into byte array {:?}", x, xs);
    let xd: Vec<f32> = bincode::deserialize(&xs).unwrap();
    assert_eq!(x, xd);

    let x: (i32, &str, f32, bool) = (1, "hello", 4.5, true);
    let xs = bincode::serialize(&x).unwrap();
    println!("tuple {:?} serializes into byte array {:?}", x, xs);
    let xd: (i32, &str, f32, bool) = bincode::deserialize(&xs).unwrap();
    assert_eq!(x, xd);

    let x = ((1u8, 2u16), (3.141f32, 'a'), true);
    let xs = bincode::serialize(&x).unwrap();
    println!("nested tuple {:?} serializes into byte array {:?}", x, xs);
}
```

Result

```
i32 number 5 serializes into byte array [5, 0, 0, 0]
f32 number 3.14 serializes into byte array [195, 245, 72, 64]
Vec<u8> [1, 2, 3] serializes into byte array [3, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3]
Vec<f32> [3.141, 2.718, 1.618] serializes into byte array [3, 0, 0, 0, 0, 0, 0, 0, 37, 6, 73, 64, 182, 243, 45, 64, 160, 26, 207, 63]
tuple (1, "hello", 4.5, true) serializes into byte array [1, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 104, 101, 108, 108, 111, 0, 0, 144, 64, 1]
nested tuple ((1, 2), (3.141, 'a'), true) serializes into byte array [1, 2, 0, 37, 6, 73, 64, 97, 1]
```

What about structs and other custom Rust data types? Well, you just need to annotate them with `serde` and they will automagically get serialization capabilities!

```rust,noplaypen
use serde::{Serialize, Deserialize};
use bincode;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
  x: f32,
  y: f32
}

#[derive(Serialize, Deserialize, Debug)]
struct Line {
  points: Vec<Point>,
  valid: bool,
  length: f32,
  desc: String
}

fn main() {
    let point1: Point = Point {x:1.0, y:2.0};
    let point2: Point = Point {x:3.0, y:4.0};
    let point1s = bincode::serialize(&point1).unwrap();
    let point2s = bincode::serialize(&point2).unwrap();
    println!("struct Point serializes into byte array {:?}", point1s);
    println!("struct Point serializes into byte array {:?}", point2s);

    let length = ((point1.x - point2.x) * (point1.x - point2.x) + (point1.y - point2.y) * (point1.y - point2.y)).sqrt();
    let valid = if length == 0.0 { false } else { true };
    let line = Line { points: vec![point1, point2], valid: valid, length: length, desc: "a thin line".to_string() };
    let lines = bincode::serialize(&line).unwrap();
    println!("struct Line serializes into byte array {:?}", lines);

    let lined: Line = bincode::deserialize(&lines).unwrap();
    assert_eq!(lined.desc, "a thin line");
    assert_eq!(lined.points[1].x, 3.0);
}
```

Result

```
struct Point serializes into byte array [0, 0, 128, 63, 0, 0, 0, 64]
struct Point serializes into byte array [0, 0, 64, 64, 0, 0, 128, 64]
struct Line serializes into byte array [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 63, 0, 0, 0, 64, 0, 0, 64, 64, 0, 0, 128, 64, 1, 243, 4, 53, 64, 11, 0, 0, 0, 0, 0, 0, 0, 97, 32, 116, 104, 105, 110, 32, 108, 105, 110, 101]
```
