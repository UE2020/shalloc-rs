# shalloc

This crate implements the `GlobalAlloc` trait for [supahero1's allocator written in C](https://github.com/supahero1/alloc).
See the README in supahero1's repo for more details.

## Usage

```rs
#[global_allocator]
static ALLOCATOR: shalloc::Shalloc = shalloc::Shalloc;
```

## Benchmark

A benchmark is available at https://github.com/supahero1/alloc
