# cplus_demangle

## cplus_demangle
This library converts C++ mangled symbol names to human-readable strings. It is a small and safe Rust wrapper to GNU libiberty's C function `cplus_demangle`.

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
