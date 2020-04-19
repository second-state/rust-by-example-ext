# Rust by Example

[Rust][rust] is a modern systems programming language focusing on safety, speed,
and concurrency. It accomplishes these goals by being memory safe without using 
garbage collection. A big part of Rust's strength and success comes from the
large ecosystem of third party libraries, known as crates.

Rust by Example Extended Edition (RBEext) is a collection of runnable examples that illustrate how to use popular Rust
third party libraries and crates. It is designed to complement the official 
[Rust by Example (RBE)][rbe] book that focuses on the 
core language and standard libraries.
Additionally for the curious, you can also [check out the source code for this site][home].

> **Note:** Many examples in this book are directly runnable from the web page. Just click on the triangle button at the top right of an example and the results are displayed under it. However, the [Rust playground](https://play.rust-lang.org/) does not support all third party crates. It supports [the top 100](https://github.com/integer32llc/rust-playground/blob/master/compiler/base/Cargo.toml). For crates or features that are not supported by the playground, the code exmple would NOT be runnable on the page. In that case, we show the results next to the examples, and link to [cargo project source code](https://github.com/second-state/rust-by-example-ext/tree/master/examples/) for the example.

Now let's begin!

- [Serialization](serde.md) - Serialization and deserialization are key to data exchange between Rust programs and the rest of the world. The `serde` crate is the de facto standard here.

- [Random numbers](rand.md) - It is surprisingly difficult to get high quality random numbers for your application. The `rand` crate can help.

- [WebAssembly](webassembly.md) - It is very popular to run Rust apps in WebAssembly, learn why and how.

[rust]: https://www.rust-lang.org/
[rbe]: https://doc.rust-lang.org/rust-by-example/
[home]: https://github.com/second-state/rust-by-example-ext