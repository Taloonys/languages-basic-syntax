mod modules;                        // <-- filename without `.rs`
                                    // .. some nesting methodology modules requires `pub`


/**
 * Rust (cargo) would search this module:
 * - inside `{}` before `mod...`
 * - in current dir `modules.rs`
 * - in `modules/mod.rs`            ~ aka special module file
 */


fn main() {
    access_module::test();
}