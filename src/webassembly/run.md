# Run a simple WebAssembly program

In the previous article, we showed you how to compile a Rust program
into a WebAssembly bytecode file.
The easiest way to run a WebAssembly bytecode file is to simply load
it into a web browser. Most modern web browsers already support
WebAssembly.

The code example below shows how to create a WebAssembly VM, load the 
bytecode application, export the Rust function in the bytecode into Javascript,
and then call this Rust / WebAssembly function from Javascript.
All those steps could be done in the browser's Javascript console.

```
const response = await fetch('my_project_name.wasm');
const buffer = await response.arrayBuffer();
const module = await WebAssembly.compile(buffer);
const instance = await WebAssembly.instantiate(module);
const exports = instance.exports;
const triple = exports.triple;
```

In the browser Javascript console, you can now call the Rust `triple()` function
in WebAssembly and triple your input number. The code below returns number `30`.

```
triple(10);
```

Of course, you can just easily load Rust compiled WebAssembly functions from
your web page. Check out [a full example here](https://github.com/second-state/wasm-learning/tree/master/browser/triple).


