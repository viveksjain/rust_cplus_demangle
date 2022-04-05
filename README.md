# cplus_demangle

## cplus_demangle
[![crates.io](https://img.shields.io/crates/v/cplus_demangle)](https://crates.io/crates/cplus_demangle) [![docs.rs](https://docs.rs/cplus_demangle/badge.svg)](https://docs.rs/cplus_demangle/latest/cplus_demangle/)

This library converts C++ mangled symbol names to human-readable strings. It is a safe Rust wrapper to GNU libiberty's C function `cplus_demangle`. I found it much faster and more robust than other Rust-native implementations that I found.

### Example
Suppose you compile the following C++ program:
```cpp
namespace test {
  void myfn(int x) { }
}
```

In the resulting binary, the symbol that gets generated for `myfn` is `_ZN4test4myfnEi`. We can convert it back with this Rust code:
```rust
assert_eq!(cplus_demangle::demangle("_ZN4test4myfnEi").unwrap(), "test::myfn(int)");
```
