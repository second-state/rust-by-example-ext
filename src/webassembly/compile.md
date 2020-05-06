# Compile targets for WebAssembly

There are several Rust compiler targets for WebAssembly. They differ
because different WebAssembly VMs supports different extensions. 
For example, some WebAssembly VMs have access the WASI extension that allows
access to system resources such as the file system.
The `wasm32` target is the most general target that produces plain WebAssembly
bytecode that can run on any WebAssembly VM.

To add the `wasm32` target to the compiler toolchain, do the following on your operating system's command line.

```
$ rustup target add wasm32-unknown-unknown
$ rustup override set nightly
$ rustup target add wasm32-unknown-unknown --toolchain nightly
```

Now, create a Rust library project in cargo, and create a public function like the following. Check out the [complete example source code here](https://github.com/second-state/wasm-learning/tree/master/browser/triple).

```
#[no_mangle]
pub extern fn triple(x: i32) -> i32 {
  return 3 * x;
}
```

You can build the project into WebAssembly bytecode using `cargo`.

```
$ cargo +nightly build --target wasm32-unknown-unknown --release
```

The compiled WebAssembly bytecode application is a single file with the `wasm` extension. For example, here is the `wasm` file from our example `target/wasm32-unknown-unknown/release/my_project_name.wasm`.


