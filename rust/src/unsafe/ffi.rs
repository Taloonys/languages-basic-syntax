/**
 * For importing stuff from different languages Rust use FFI lib
 */


extern "C" {                                            // import from C-language `printf`
    fn printf(format_str:&[u8;13]);                     // we just say, that we are passing 13 UTF-8 symbols
}


fn main() {
    unsafe {
        printf(b"1234567890123");               // using printf from C, and pass 13 UTF-8 symbols
    }
}