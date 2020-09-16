//! # cplus_demangle
//! This library converts C++ mangled symbol names to human-readable strings. It is a safe Rust wrapper to GNU libiberty's C function `cplus_demangle`.
//! 
//! ## Example
//! Suppose you compile the following C++ program:
//! ```cpp
//! namespace test {
//!   void myfn(int x) { }
//! }
//! ```
//! 
//! In the resulting binary, the symbol that gets generated for `myfn` is `_ZN4test4myfnEi`. We can convert it back with this Rust code:
//! ```rust
//! assert_eq!(cplus_demangle::demangle("_ZN4test4myfnEi").unwrap(), "test::myfn(int)");
//! ```

use libc::{c_char, c_int};
use std::ffi::{CStr, CString};

extern "C" {
    fn cplus_demangle_wrapper(mangled_name: *const c_char, show_params: c_int, show_ansi: c_int) -> *mut c_char;
}

#[derive(Debug)]
pub struct Error(&'static str);

/// Description of options from demangle.h:
/// ```
/// #define DMGL_PARAMS      (1 << 0)       /* Include function args */
/// #define DMGL_ANSI        (1 << 1)       /* Include const, volatile, etc */
/// ```
pub struct Options {
    pub show_params: bool,
    pub show_ansi: bool,
}

impl Options {
    pub fn default() -> Options {
        Options {
            show_params: true,
            show_ansi: true,
        }
    }
}

/// Demangle the given name, with default options.
pub fn demangle(mangled_name: &str) -> Result<String, Error> {
    demangle_with_options(mangled_name, Options::default())
}

/// Demangle the given name with the specified options.
///
/// Fails if: the name contains a null character, or demangling fails.
pub fn demangle_with_options(mangled_name: &str, options: Options) -> Result<String, Error> {
    let mangled_name = match CString::new(mangled_name) {
        Ok(mangled_name) => mangled_name,
        Err(std::ffi::NulError { .. }) => return Err(Error("mangled_name contains null")),
    };
    let result: *mut c_char = unsafe {
        cplus_demangle_wrapper(mangled_name.as_ptr(), options.show_params as i32, options.show_ansi as i32)
    };
    if result.is_null() {
        // Unfortunately cplus_demangle appears to give us precisely 0 helpful
        // error info, we have to go with a generic message.
        return Err(Error("Failed to demangle"));
    }
    let demangled = unsafe { CStr::from_ptr(result) };
    Ok(demangled.to_str().unwrap().to_owned())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(
            crate::demangle("_ZNK5boost16cpp_regex_traitsIcE7isctypeEcj").unwrap(),
            "boost::cpp_regex_traits<char>::isctype(char, unsigned int) const"
        );
        assert_eq!(
            crate::demangle_with_options("_ZNK5boost16cpp_regex_traitsIcE7isctypeEcj", crate::Options {
                show_params: false,
                show_ansi: true,
            }).unwrap(),
            "boost::cpp_regex_traits<char>::isctype"
        );
    }
}
