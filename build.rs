fn main() {
  cc::Build::new()
    .file("src/demangle.c")
    .compile("libdemangle.a");
  // Cannot use #[link] macro in lib.rs because it adds -liberty before linking
  // with our libdemangle.a which will fail.
  println!("cargo:rustc-link-lib=iberty");
}
