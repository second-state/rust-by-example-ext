# Random numbers

A lot of applications require random numbers. The [`rand`](https://crates.io/crates/rand) crate is a very popular library in Rust to generate random numbers. 
It supports a wide variety of random number generators and distributions, each with a different performance and security trade off. 
For more details, you can read the [Rust Rand Book](https://rust-random.github.io/book/intro.html).
To use the `rand` crate, just do the following in your `Cargo.toml` file.

```
[dependencies]
rand = "0.7.3"
```

## Get a random number

To get a random number, you can simply do the following.

```rust,editable
# extern crate rand;

fn main() {
    let i: i32 = rand::random();
    println!("The random i32 is {}", i);
}
```

The `random()` is smart enough to know the primitive type it is **supposed** to generate. Check out the example below.

```rust,editable
# extern crate rand;

fn main() {
    let x: u8 = rand::random();
    println!("The random u8 is {}", x);

    let x: f64 = rand::random();
    println!("The random f64 is {}", x);

    let x:bool = rand::random();
    println!("The random bool {}", x);
}
```

What about generating a random number within a range? For that, you need to
create a random number generator and call its `gen_range()` function.

```rust,editable
# extern crate rand;

use rand::thread_rng;
use rand::Rng;

fn main() {
    let mut rng = thread_rng();
    let y: f64 = rng.gen_range(-10.0, 10.0);
    println!("Number from -10. to 10.: {}", y);
    println!("Number from 0 to 9: {}", rng.gen_range(0, 10));
}
```

## Get a series of random numbers

In order to get a series of random numbers, you could call the `random()` function
multiple times. But that is slow since every time it needs to instantiate and seed
a new random number generator. It is faster to create the generator once and 
call its `gen()` function repeatedly.

```rust,editable
# extern crate rand;

use rand::thread_rng;
use rand::Rng;

fn main() {
    let mut rng = thread_rng();
    for i in 1..10 {
        println!("Random number #{}: {}", i, rng.gen_range(0, 100));
    }
}
```

The generator can quickly fill an array with random integers.

```rust,editable
# extern crate rand;

use rand::thread_rng;
use rand::Rng;

fn main() {
    let mut arr = [0i32; 9];
    thread_rng().try_fill(&mut arr[..]);
    println!("Random number array {:?}", arr);
}
```

Another neat feature of the generator is that it can generate random numbers 
from a probability distribution.

```rust,editable
# extern crate rand;

use rand::thread_rng;
use rand::Rng;

fn main() {
    let mut rng = thread_rng();
    let distr = rand::distributions::Uniform::new_inclusive(1, 100);
    let mut nums = [0i32; 3];
    for x in &mut nums {
        *x = rng.sample(distr);
    }
    println!("Some numbers: {:?}", nums);
}
```

## WebAssembly

Under the hood, the `rand` crate uses the operating system's native random
functions to seed the random number generator. This initial entropy seed
comes from the hardware noise. However, when Rust programs inside virtual
machines like WebAssembly, it does not have access to native random hardware.

When the `rand` crate is compiled for WebAssembly, it has a special feature
to use JavaScript's random function to seed the generator. This works when
your WebAssembly application is running inside a JavaScript host, such as in
a web browser or in Node.js. To enable that, do the following in your `Cargo.toml`.

```
[dependencies]
rand = { version = "0.7.3", features = ["wasm-bindgen"] }
```

Then you must use one of the [`wasm-bindgen`](https://rustwasm.github.io/docs/wasm-bindgen/) compatible tools to instrument 
your Rust code to call external JavaScript and a JavaScript shim to be called 
from Rust.

- The [`ssvmup`](https://www.npmjs.com/package/ssvmup) and [`ssvm`](https://www.npmjs.com/package/ssvm) tools enable high performance Rust + JavaScript hybrid applications in Node.js. [Check out this tutorial](https://cloud.secondstate.io/server-side-webassembly/getting-started).
- The [`wasm-pack`](https://rustwasm.github.io/wasm-pack/) tool can generate Rust and JavaScript code for the V8 engine.



