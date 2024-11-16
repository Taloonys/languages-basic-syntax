mod modules;                        // <-- filename without `.rs`
                                    // .. some nesting methodology modules requires `pub`


/**
 * Rust (cargo) would search this module:
 * - inside `{}` before `mod...`
 * - in current dir `modules.rs`
 * - in `modules/mod.rs`            ~ aka special module file
 */

 /**
  * `Crate` is synonim to `translation unit`, like any `.rs` file is crate, it could be compiled to executable binary or lib
  * `Packet` contains multiple `crates` and composed with `cargo.toml`,
  *     there are like 2 "main files" for `crate`: lib.rs & main.rs
  */

fn main() {
    access_module::test();
}