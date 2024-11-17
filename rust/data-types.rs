/**
 * Warning: Rust will warning unused variables: `value` -> `_value`
 * There are no `void` type, but there could be `unit` type ~ also could be used as `()` but u can't use it directly
 */

fn params() {
    let age: u32;               // immutable by default
    age = 50;

    let id: i32 = 444;          // i32 is like a default int

    let text = "Text";          // auto type deduction

    let mut x = 50;             // mutable data by `mut`
    x = 45;

    let a: bool = true;

    let _b = 6000;              // `aka unused variable`
}


fn casting() {
    let a: i32  = 5;
    let b: i16  = 10 as i16;    // only explicit casts
}


/**
 * Operations are:
 * +, -, *, /, %
 * +=, -=, *=, /=, %=
 */


 const GLOBAL_PI : f32 = 3.1415;    // could be in global namespace
// let const_val aaa: i32 = 9;      /*<-- err, can't be in a global namespace */

 fn const_value() {
    const PI : f32 = 3.14;          // it's a compile time constant!
 }


 fn ranges() {
    let numerals    = 0..9;         // [0, 9) , `9` is excluded
    let numerals1   = 0..=9;        // [0, 9], `9` is included
    for num in 0..9 {
        print!("{num} ");
    }

    let numbers = std::ops::Range{  // full explicit way
        start   : 1, 
        end     : 9 
    };
 }