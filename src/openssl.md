# Cryptography with openssl

[OpenSSL](https://www.openssl.org/) is an widely used open source library to perform cryptographic
operations. We can use it to encrypt, decrypt, digest, and sign data, 
as well as to make SSL/TLS secure connections through the Internet.

In this chapter, we will cover the [`openssl`](https://crates.io/crates/openssl) crate. 
You can [read more documentation about it](https://docs.rs/openssl/0.10.29/openssl/).

> Since the `openssl` crate relies on the native operating system's OpenSSL library to perform cryptographic operations, it cannot be compiled into WebAssembly targets.

Let's [start with an example](openssl/rsa.md) to create a pair of RSA keys, and use them to encrypt and decrypt data.

