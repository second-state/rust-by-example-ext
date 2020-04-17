# Serialization and deserialization

Serialization is a core language feature in the era of web applications. When one program needs to talk to another program
over the Internet, it needs to serialize its data into a format that can be transmitted through the network. The receiving
program uses deserialization to reconstruct the data.

In Rust, most applications use the [`serde`](https://crates.io/crates/serde) crate to manage serialization and deserialization. In this chapter, we will cover how to serialize typed Rust data into [JSON strings](serde/json.md) 
or [byte arrays](serde/binary.md). We will also discuss how to serialize third party structs in libraries.

Checkout the [offical documentation](https://serde.rs/) for the `serde` crate.
