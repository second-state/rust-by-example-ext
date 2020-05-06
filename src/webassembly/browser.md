# Rust Javascript binding through WebAssembly

In the previous articles, we showed how to compile a Rust function into
WebAssembly, and then call this function from Javascript in a web browser.
However, that approach has some severe limitations. Specifically, WebAssembly
has very limited support for data types. When a WebAssembly function is 
exported to Javascript, the input parameters and return values are limited
32 bit integers. You cannot even pass or return string values to the WebAssembly
function! You could even do a Hello World!

Fortunately, the `wasm-bindgen` project provides binding between Rust and
Javascript. An easy way to `wasm-bindgen` in a browser project is to the 
`wasm-pack` tool. Here is how to install `wasm-pack` through the npm package manager.

```
$ npm install -g wasm-pack
```

In your project's `Cargo.toml`, add dependency for the `wasm-bindgen` crate.

```
[dependencies]
wasm-bindgen = "0.2.50"
```

Below is a Rust library function we wrote. As you can see it now takes and returns `String` values. The function must be annotated with `#[wasm_bindgen]` in order for the Rust compiler toolchain to call `wasm-bindgen` to generate the necessary shim code that binds Rust and Javascript through WebAssembly.

```
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn say(s: String) -> String {
  let r = String::from("hello ");
  return r + &s;
}
```

Build the `wasm` bytecode application and its Javascript helper files via `wasm-pack`.
The generated files are in the `pkg` directory.

```
$ wasm-pack build --target web
```

From a web browser's Javascript console, you can load the generated Javascript file, export the Rust function, and call it.

```
import init, { say } from 'pkg/hello_lib.js';
init();
say("Michael");
```

The result is `Hello Michael`.

To see the complete source code and run it on a web page, [checkout here](https://github.com/second-state/wasm-learning/tree/master/browser/hello).

