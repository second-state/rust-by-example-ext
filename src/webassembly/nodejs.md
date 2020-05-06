# Rust and WebAssembly on the server side

In the previous article, we discussed how to run Rust functions in web browsers.
In this article, we will show you how to do this on the server side.
We believe that Rust and WebAssembly [brings a lot of benefits](https://cloud.secondstate.io/server-side-webassembly/why) to server side
applications.

On the server side, we use the open source [Second State VM (SSVM)](https://github.com/second-state/SSVM) to execute
Rust and WebAssembly functions in the Node.js environment. We still use
the `wasm-bindgen` crate to support binding between Rust and
Javascript. An easy way to `wasm-bindgen` in a SSVM Node.js project is to the 
`ssvmup` tool. Here is how to install `ssvmup` through the npm package manager.

```
$ npm install -g ssvmup
```

In your project's `Cargo.toml`, add dependency for the `wasm-bindgen` crate.

```
[dependencies]
wasm-bindgen = "=0.2.61"
```

## Hello world

Below is a Rust hello world function we wrote. The function must be annotated with `#[wasm_bindgen]` in order for the Rust compiler toolchain to call `wasm-bindgen` to generate the necessary shim code that binds Rust and Javascript through WebAssembly.

```
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn say(s: String) -> String {
  let r = String::from("hello ");
  return r + &s;
}
```

Build the `wasm` bytecode application and its Javascript helper files via `ssvmup`.
The generated files are in the `pkg` directory.

```
$ ssvmup build
```

In a Javascript application, you can load the generated Javascript file, export the Rust function, and call it.

```
const { say } = require('pkg/hello_lib.js');
say("Michael");
```

You can now run the Javascript application from Node.js command line.

```
$ node app.js
Hello Michael
```

To see the complete source code and run it in a Node.js server, [checkout here](https://github.com/second-state/wasm-learning/tree/master/nodejs/hello).

## Beyond simple arguments

Using the `serde` tool, we can pass and return arbitary Javascript values to Rust via the SSVM WebAssembly runtime. First, add `serde` to your dependencies.

```
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
wasm-bindgen = "=0.2.61"
```

The Rust function takes two floating point numbers, and returns the product of
the two. Notice that it parses the input and return values in JSON.

```
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn area(sides: &str) -> String {
  let s: (f32, f32) = serde_json::from_str(&sides).unwrap();
  let a = s.0 * s.1;
  return serde_json::to_string(&a).unwrap();
}
```

Build and create the `pkg` folder for the compiled `wasm` bytecode file and the Javascript shim file `my_project_name.js`.

```
$ ssvmup build
```

The Javascript calling program in Node.js looks like the following.

```
const { area } = require('pkg/my_project_name.js');
var x = [10., 5.];
console.log( area(JSON.stringify(x)) );
```

Run the Node.js app shows the following result.

```
$ node app.js
50.0
```

## Structs and objects

Using the `serde` tool, we can pass and return Javascript objects and arrays to Rust functions. Javascript objects are mapped to Rust structs.

The Rust function `draw()` takes two JSON strings, each representing a `Point` struct, and returns a JSON string representing a `Line` struct.

```
use wasm_bindgen::prelude::*;
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

#[wasm_bindgen]
pub fn draw(points: &str) -> String {
  let ps: (Point, Point, String) = serde_json::from_str(&points).unwrap();
  let length = ((ps.0.x - ps.1.x) * (ps.0.x - ps.1.x) + (ps.0.y - ps.1.y) * (ps.0.y - ps.1.y)).sqrt();

  let valid = if length == 0.0 { false } else { true };
  let line = Line { points: vec![ps.0, ps.1], valid: valid, length: length, desc: ps.2 };
  return serde_json::to_string(&line).unwrap();
}
```

Build and create the `pkg` folder for the compiled `wasm` bytecode file and the Javascript shim file `my_project_name.js`.

```
$ ssvmup build
```

The Javascript calling program in Node.js looks like the following.

```
const { draw } = require('pkg/my_project_name.js');
var x = [{x:1.5, y:3.8}, {x:2.5, y:5.8}, "A thin red line"];
console.log( draw(JSON.stringify(x)) );
```

Run the Node.js app shows the following result.

```
$ node app.js
{"points":[{"x":1.5,"y":3.8},{"x":2.5,"y":5.8}],"valid":true,"length":2.2360682,"desc":"A thin red line"}
```

## Binary data

A server side function often needs to process binary data directly. The SSVM
toolchain supports that use case out of the box.
In this example, we will show you how to create a Rust function on the server
to compute a SHA3 cryptographic digest for an arbitary input binary data array. First, add 
the `sh3` crate as a dependency.

```
[dependencies]
sha3 = "0.8.2"
wasm-bindgen = "=0.2.61"
```

Below is the Rust function that computes the SHA3 digest value. Notice that both
its input and return values are byte arrays.

```
use wasm_bindgen::prelude::*;
use sha3::{Digest, Sha3_256};

#[wasm_bindgen]
pub fn sha3_digest(v: &[u8]) -> Vec<u8> {
  return Sha3_256::digest(&v).as_slice().to_vec();
}
```

Build and create the `pkg` folder for the compiled `wasm` bytecode file and the Javascript shim file `my_project_name.js`.

```
$ ssvmup build
```

The Javascript calling program in Node.js looks like the following.

```
const { sha3_digest } = require('pkg/my_project_name.js');
console.log( sha3_digest(encoder.encode("This is an important message")) );
```

Run the Node.js app shows the following result.

```
$ node app.js
000000  57 1b e7 d1 bd 69 fb 31 9f 0a d3 fa 0f 9f 9a b5  W.çÑ½iû1..Óú...µ
000010  2b da 1a 8d 38 c7 19 2d 3c 0a 14 a3 36 d3 c3 cb  +Ú..8Ç.-<..£6ÓÃË
```

