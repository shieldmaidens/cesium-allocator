# Cesium Allocator

![docs.rs](https://img.shields.io/docsrs/cesium-allocator) ![Crates.io](https://img.shields.io/crates/d/cesium-allocator)


This is an idiomatic and quality of life wrapper on top of [`mimalloc`](https://github.com/microsoft/mimalloc). The only changes to the underlying library are basic type wrappers. The underlying FFI library, [`cesium-libmimalloc-sys`](./libmimalloc-sys), is based very heavily on Purple Protocol's [`mimalloc_rust`](https://github.com/purpleprotocol/mimalloc_rust) wrapper, but with some reorganization as there are a lot of methods.

Happy allocating!
