# Serialize into JSON strings

To use the `serde` crate, you just need to add the following dependencies to your `Cargo.toml` file.

```
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

The example below shows how to serialize a simple Rust primitive data type `i32` into a JSON string, and then deserialize it back. Run it! You can pass this JSON string to other Rust programs or Internet applications written in other languages.

```rust,editable
# extern crate serde;

fn main() {
    let x: i32 = 5;
    let xs = serde_json::to_string(&x).unwrap();
    println!("i32 number {} serializes into string {}", x, xs);
    let xd: i32 = serde_json::from_str(&xs).unwrap();
    assert_eq!(x, xd);
}
```

Here are more examples showing the serialization and deserialization of Rust primitive data types. Rust arrays and tuples are all serialized to JSON arrays. Edit the code below to try more Rust types, and run it to see the results.

```rust,editable
# extern crate serde;

fn main() {
    let x: i32 = 5;
    let xs = serde_json::to_string(&x).unwrap();
    println!("i32 number {} serializes into string {}", x, xs);
    let xd: i32 = serde_json::from_str(&xs).unwrap();
    assert_eq!(x, xd);

    let x: f32 = 3.14;
    let xs = serde_json::to_string(&x).unwrap();
    println!("f32 number {} serializes into string {}", x, xs);

    let x: Vec<u8> = vec![1, 2, 3];
    let xs = serde_json::to_string(&x).unwrap();
    println!("Vec<u8> {:?} serializes into string {}", x, xs);
    let xd: Vec<u8> = serde_json::from_str(&xs).unwrap();
    assert_eq!(x, xd);

    let x: Vec<f32> = vec![3.141, 2.718, 1.618];
    let xs = serde_json::to_string(&x).unwrap();
    println!("Vec<f32> {:?} serializes into string {}", x, xs);
    
    let x: (i32, &str, f32, bool) = (1, "hello", 4.5, true);
    let xs = serde_json::to_string(&x).unwrap();
    println!("tuple {:?} serializes into string {}", x, xs);
    let xd: (i32, &str, f32, bool) = serde_json::from_str(&xs).unwrap();
    assert_eq!(x, xd);

    let x = ((1u8, 2u16), (3.141f32, 'a'), true);
    let xs = serde_json::to_string(&x).unwrap();
    println!("nested tuple {:?} serializes into string {}", x, xs);
}
```

What about structs and other custom Rust data types? Well, you just need to annotate them and they will automagically get serialization capabilities! Run the example below and you can see the JSON string representation of these Rust structs.

```rust,editable
# extern crate serde;

use serde::{Serialize, Deserialize};

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
    let point1s = serde_json::to_string(&point1).unwrap();
    let point2s = serde_json::to_string(&point2).unwrap();
    println!("struct Point serializes into string {}", point1s);
    println!("struct Point serializes into string {}", point2s);

    let length = ((point1.x - point2.x) * (point1.x - point2.x) + (point1.y - point2.y) * (point1.y - point2.y)).sqrt();
    let valid = if length == 0.0 { false } else { true };
    let line = Line { points: vec![point1, point2], valid: valid, length: length, desc: "a thin line".to_string() };
    let lines = serde_json::to_string(&line).unwrap();
    println!("struct Line serializes into string {}", lines);

    let lined: Line = serde_json::from_str(&lines).unwrap();
    assert_eq!(lined.desc, "a thin line");
    assert_eq!(lined.points[1].x, 3.0);
}
```

