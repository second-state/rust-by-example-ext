# Call Javascript from Rust

In the previous article, we discussed how to call Rust functions from
Javascript. How about the other way around? The `nodejs-helper` crate
enables Rust functions to call Javascript functions in Node.js.

> To see the complete source code and run it in Node.js, [checkout here](https://github.com/second-state/wasm-learning/tree/master/nodejs/nodejs_example).

In your project's `Cargo.toml`, add dependency for the [`nodejs-helper`](https://github.com/second-state/nodejs-helper) and `wasm-bindgen` crates.

```
[dependencies]
wasm-bindgen = "=0.2.61"
nodejs-helper = "0.0.3"
```

## Console and time

Recall that WebAssembly is a very simple and standalone virtual machine. It
has no access the operating system's standard input / output, as well as 
features such as the system clock. In our Rust function, we can
rely on the Javascript functions in Node.js to access those festures.

```
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn show_now() {
  nodejs_helper::console::log("Timestamp now: ");
  nodejs_helper::console::log(&nodejs_helper::date::timestamp());
}

#[wasm_bindgen]
pub fn utc_now() {
  nodejs_helper::console::log("UTC time: ");
  nodejs_helper::console::log(&nodejs_helper::date::utc_string());
}

#[wasm_bindgen]
pub fn my_time(tz: &str) {
  nodejs_helper::console::log(tz);
  nodejs_helper::console::log(&nodejs_helper::date::format_date("en-US", "long", "numeric", "long", "numeric", tz, "short"));
}
```

Build the `wasm` bytecode application and its Javascript helper files via `ssvmup`.
The generated files are in the `pkg` directory.

```
$ ssvmup build
```

In a Javascript application, you can load the generated Javascript file, export the Rust function, and call it.

```
const { show_now, utc_now, my_time } = require('pkg/nodejs_example.js');

show_now();
utc_now();
my_time("America/Chicago");
```

You can now run the Javascript application from Node.js command line.

```
$ node date.js
Timestamp now:
1588013800826
UTC time:
Mon, 27 Apr 2020 18:56:40 GMT
America/Chicago
Monday, April 27, 2020, CDT
```

## File system access

The Rust functions in this section read an image file from the local file system, resize it, and write back to the file system. It also uses the Javascript console tool to measure the time spent on each task.

```
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
wasm-bindgen = "=0.2.61"
image = "0.23.0"
```

The Rust function takes
a JSON string input that contains the image file names and resized dimensions.

```
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
#[derive(Copy, Clone, Debug)]
pub struct Dimension {
  pub width: u32,
  pub height: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Picture {
  pub dim: Dimension,
  pub raw: Vec<u8>,
}

#[wasm_bindgen]
pub fn resize_file(input: &str) {
  // Use JSON to pass multiple call arguments
  let p: (Dimension, String, String) = serde_json::from_str(input).unwrap();

  nodejs_helper::console::time("Resize file");
  let raw = nodejs_helper::fs::read_file_sync(&p.1);
  nodejs_helper::console::time_log("Resize file", "Done reading");
  let src = Picture {
    dim: p.0,
    raw: raw,
  };
  let target = resize_impl(&src);
  nodejs_helper::console::time_log("Resize file", "Done resizing");

  nodejs_helper::fs::write_file_sync(&p.2, &target.raw);
  nodejs_helper::console::time_log("Resize file", "Done writing");
  nodejs_helper::console::time_end("Resize file");
}

pub fn resize_impl(src: &Picture) -> Picture {
  // ... use the img crate to resize ...
}
```

Build and create the `pkg` folder for the compiled `wasm` bytecode file and the Javascript shim file `my_project_name.js`.

```
$ ssvmup build
```

The Javascript calling program in Node.js looks like the following.

```
const { resize_file } = require('../pkg/nodejs_example.js');

const dim = {
    width: 100,
    height: 100
};

resize_file(JSON.stringify([dim, 'cat.png', `test.png`]));
```

Run the Node.js app shows the following result.

```
$ node image.js
Resize file: 5.603ms Done reading
Resize file: 1506.694ms Done resizing
Resize file: 1507.634ms Done writing
Resize file: 1507.977ms
```

## Sqlite database access

First, you must have the `better-sqlite3` module installed in
your Node.js setup. The Rust function will access a sqlite database through this module.

```
$ npm i better-sqlite3
```

The Rust functions to create, update, and query a Sqlite database on the local
file system are as follows.


```
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
  pub id: u32,
  pub full_name: String,
  pub created: String,
}

#[wasm_bindgen]
pub fn create_sqlite(path: &str) {
  let sql_create = "
CREATE TABLE users (
  id INTEGER PRIMARY KEY NOT NULL,
  full_name TEXT NOT NULL,
  created DATE NOT NULL
);";
  let sql_insert = "
INSERT INTO users
VALUES
(1, 'Bob McFett', '32-01-01'),
(2, 'Angus Vader', '02-03-04'),
(3, 'Imperator Colin', '01-01-01');";

  nodejs_helper::sqlite3::create(path);
  nodejs_helper::sqlite3::update(path, sql_create);
  nodejs_helper::sqlite3::update(path, sql_insert);
}

#[wasm_bindgen]
pub fn query_sqlite(path: &str) {
  let sql_query = "SELECT * FROM users;";
  let rows: String = nodejs_helper::sqlite3::query(path, sql_query);
  let users: Vec<User> = serde_json::from_str(&rows).unwrap();
  for user in users.into_iter() {
    nodejs_helper::console::log(&(user.id.to_string() + " : " + &user.full_name));
  }
}
```

Build and create the `pkg` folder for the compiled `wasm` bytecode file and the Javascript shim file `my_project_name.js`.

```
$ ssvmup build
```

The Javascript calling program in Node.js looks like the following.

```
const { create_sqlite, query_sqlite } = require('pkg/nodejs_example.js');
create_sqlite("test.sqlite");
query_sqlite("test.sqlite");
```

Run the Node.js app shows the following result.

```
$ node db.js
1 : Bob McFett
2 : Angus Vader
3 : Imperator Colin
```

## HTTP request

First, you must have the `sync-request` module installed in
your Node.js setup. The Rust function will make synchronous HTTP requests through this module.

```
$ npm i sync-request
```

The Rust functions to access web services via HTTP/HTTPS and then save content on the local file system are as follows.

```
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn fetch(url: &str) {
  let content = nodejs_helper::request::fetch_as_string(url);
  nodejs_helper::console::log(url);
  nodejs_helper::console::log(&content);
}

#[wasm_bindgen]
pub fn download(url: &str, path: &str) {
  let content = nodejs_helper::request::fetch(url);
  nodejs_helper::fs::write_file_sync(path, &content);
}
```

Build and create the `pkg` folder for the compiled `wasm` bytecode file and the Javascript shim file `my_project_name.js`.

```
$ ssvmup build
```

The Javascript calling program in Node.js looks like the following.

```
const { fetch, download } = require('../pkg/nodejs_example.js');

fetch("https://raw.githubusercontent.com/second-state/nodejs-helper/master/LICENSE");
download("https://www.secondstate.io/", "test.html");
```

Run the Node.js app shows the following result.

```
$ node http.js
https://raw.githubusercontent.com/second-state/nodejs-helper/master/LICENSE
MIT License

Copyright (c) 2020 Second State

Permission is hereby granted, free of charge, to any person obtaining a copy
... ...
```

