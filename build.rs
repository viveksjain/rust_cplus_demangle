fn main() {
    // docs.rs build env does not seem to have libiberty
    if let Ok(_) = std::env::var("DOCS_RS") {
        return;
    }

    cc::Build::new()
        .file("src/demangle.c")
        // Some distros (e.g. Ubuntu) have `demangle.h` under this folder,
        // others have it directly inside `/usr/include` which is in default
        // include path. This approach works across both.
        .include("/usr/include/libiberty")
        .compile("libdemangle.a");
    // Cannot use #[link] macro in lib.rs because it adds -liberty before linking
    // with our libdemangle.a which will fail.
    println!("cargo:rustc-link-lib=iberty");
}
