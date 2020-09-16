use libc::{c_char, c_int};
use std::ffi::{CStr, CString};

extern "C" {
    fn cplus_demangle_wrapper(mangled_name: *const c_char, show_params: c_int, show_ansi: c_int) -> *mut c_char;
}

#[derive(Debug)]
pub struct Error(&'static str);

/// Description of options from demangle.h:
/// #define DMGL_PARAMS      (1 << 0)       /* Include function args */
/// #define DMGL_ANSI        (1 << 1)       /* Include const, volatile, etc */
pub struct Options {
    show_params: bool,
    show_ansi: bool,
}

impl Options {
    pub fn default() -> Options {
        Options {
            show_params: true,
            show_ansi: true,
        }
    }
}

pub fn demangle(mangled_name: &str) -> Result<String, Error> {
    demangle_with_options(mangled_name, Options::default())
}

pub fn demangle_with_options(mangled_name: &str, options: Options) -> Result<String, Error> {
    let mangled_name = match CString::new(mangled_name) {
        Ok(mangled_name) => mangled_name,
        Err(std::ffi::NulError { .. }) => return Err(Error("mangled_name contains null")),
    };
    let result: *mut c_char = unsafe {
        cplus_demangle_wrapper(mangled_name.as_ptr(), options.show_params as i32, options.show_ansi as i32)
    };
    if result.is_null() {
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
