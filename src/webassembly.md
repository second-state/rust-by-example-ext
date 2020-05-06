# WebAssembly

Besides operating system specific native binaries, Rust programs can also be compiled into managed 
code that runs inside containers or virtual machines. The reason to use managed code is runtime safety. That allows un-trusted
Rust programs to run in environments such as web browsers and servers. Compared with native binaries, managed
containers are also much easier to provision, limit access to resources, start, and stop on demand.

A popular managed code compiler target is the WebAssembly virtual machine. With WebAssembly, your Rust programs can run side-by-side with Javascript in
web browsers and servers. In this chapter, we will show you how.
