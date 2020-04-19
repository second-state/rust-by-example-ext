# RSA public key encryption

To use the `openssl` crate, you just need to add the following dependencies to your `Cargo.toml` file.

```
[dependencies]
openssl = "0.10.28"
```

The example below generates an RSA public and private key pair, and
encrypt the keys with a phassphrase.

```rust,editable
# extern crate openssl;

use openssl::rsa::{Rsa, Padding};
use openssl::symm::Cipher;

fn main() {
    let passphrase = "rust_by_example";

    let rsa = Rsa::generate(1024).unwrap();
    let private_key: Vec<u8> = rsa.private_key_to_pem_passphrase(Cipher::aes_128_cbc(), passphrase.as_bytes()).unwrap();
    let public_key: Vec<u8> = rsa.public_key_to_pem().unwrap();

    println!("Private key: {}", String::from_utf8(private_key).unwrap());
    println!("Public key: {}", String::from_utf8(public_key).unwrap());
}
```

Next, we can import a public key from the PEM document, and use it to encrypt
some data.

```rust,editable
# extern crate openssl;

use openssl::rsa::{Rsa, Padding};
use openssl::symm::Cipher;

fn main() {
    let passphrase = "rust_by_example";
    let public_key_pem = "1234";
    let private_key_pem = "1234";
    let data = "A quick brown fox jumps over the lazy dog.";

    // Encrypt with public key
    let rsa = Rsa::public_key_from_pem(public_key_pem.as_bytes()).unwrap();
    let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
    let _ = rsa.public_encrypt(data.as_bytes(), &mut buf, Padding::PKCS1).unwrap();
    println!("Encrypted: {}", buf);

    let data = buf;

    // Decrypt with private key
    let rsa = Rsa::private_key_from_pem_passphrase(private_key_pem.as_bytes(), passphrase.as_bytes()).unwrap();
    let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
    let _ = rsa.private_decrypt(data, &mut buf, Padding::PKCS1).unwrap();
    println!("Decrypted: {}", String::from_utf8(buf).unwrap());
}
```

