# RSA public key encryption

To use the `openssl` crate, you just need to add the following dependencies to your `Cargo.toml` file.

```
[dependencies]
openssl = "0.10.28"
```

The example below generates an RSA public and private key pair, and
encrypts the keys with a phassphrase. The outputs are text strings that
can be saved into files. Those files are called PEM (Privacy Enhanced Mail) files.

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

Next, we can import public and private keys from the PEM document.
In the example, we demonstrate how to encrypt a byte array of data using the
public key. Such encrypted data can only be decrypted by the correspding private key.


```rust,editable
# extern crate openssl;

use openssl::rsa::{Rsa, Padding};

fn main() {
    let passphrase = "rust_by_example";
    
    let public_key_pem = "-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDC+Jx89MjzbWw9PPh0dffD+i2c
J7XMioLndImQvQiNJjZ00zyxjgt4+wkual+ZHhH94HIjRIeLI+ncBEjFMa1xIzHT
exz/pvJUCsHNxNK9958zR0E997xxSf3C2Lu8BWtJG348xd5QNzb+R+i963PtcAsQ
fCu+q5gbqqtQEIjlMwIDAQAB
-----END PUBLIC KEY-----";

    let private_key_pem = "-----BEGIN RSA PRIVATE KEY-----
Proc-Type: 4,ENCRYPTED
DEK-Info: AES-128-CBC,43371B6CECDB096AC2A362FD33BF4B07

aIs3x9UBN95VJJFsd1ddYxmwAKQdFE5BJwZVYtidV+cZ4Qpmg9tdBLm8AhF5bVGR
FzAVMxTEFQgwT4o2jH2UxRkRmChwNy6aqdGteDIK6yXQK7//GMmxhbvqMmFzwdof
2E7Jkq3BQQEqMFu2CxRUPUFYRIebEIZSDCD3PoJ6p7a77qwm/KCXCbad/DqtOGkJ
wOkPH5AXLIu02MJfs+vcLswXFMlq7aaUrAv5WGt1SpKz9Co6bplSYDG7JE+906Uw
MIg4XDJTJDKCKyDaPkMydw6StvyNuZfIYUNIofulLci7yoNEGvwQHsHCaHr6n4bt
I4iC9CbkEcPbf06HAWGFfsexeLGf9mU0HVsZi83QdMhWMbOREakFU755AMvTeB8w
IMCNn55nzJlSHooKuvJAmbqBBb4+wqgwnoYQEVZmTDZxqT/eR08Zl9d1QeKB+1fw
gjZmY/10kFLnTKlWGIaLIu60ehbXxZeFbW+m1pF9uHEiIkWgkrHNjKfzWh5EyfhY
vXxWuZH92ZP/nioGzVQr00oSEPLwW1RSoAx3jPuu1EILNu7lFL896CsDZpa1Oigf
OMxk0GhMuKs4H6TlHmx5a0TOAcGYWEbnqXi+KUw7pMPFiEs1/2crFI6QfQx8R7dL
/ohKFvksPExsB196RZ1PFyMdryOr/mCqI4nBT+KzPz4zJF2iTMGq3NFQI2MvW/4g
WMwsyQtIJQviFJpYlQpOVBFaeB69oHJMxfauM8OdEU8yomFl3sAVagNxPfiWsGt4
LRsReK2BDT/pnhhZG96qSsNPwQlrwffBleTy9BGSuHHox6A7GKyVAAOMND/TY1ak
-----END RSA PRIVATE KEY-----";

    let data = "A quick brown fox jumps over the lazy dog.";

    // Encrypt with public key
    let rsa = Rsa::public_key_from_pem(public_key_pem.as_bytes()).unwrap();
    let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
    let _ = rsa.public_encrypt(data.as_bytes(), &mut buf, Padding::PKCS1).unwrap();
    println!("Encrypted: {:?}", buf);

    let data = buf;

    // Decrypt with private key
    let rsa = Rsa::private_key_from_pem_passphrase(private_key_pem.as_bytes(), passphrase.as_bytes()).unwrap();
    let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
    let _ = rsa.private_decrypt(&data, &mut buf, Padding::PKCS1).unwrap();
    println!("Decrypted: {}", String::from_utf8(buf).unwrap());
}
```

