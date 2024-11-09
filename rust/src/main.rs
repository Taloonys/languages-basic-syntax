//
// Rust decide at compile time, which traits object has, but we can dynamicly dispatch which trait we a gonna stick to
//

// from associative-traits.rs *

use std::io::{self, Read};

fn input_keyboard() {
    let mut input_word = String::new();
    println!("Input any word");

    let _result = io::stdin().read_line(&mut input_word);    // read until new line
    println!("Got {input_word}");
}


fn main() {
    input_keyboard();
}