/**
 * Warning: Rust will warning unused variables: `value` -> `_value`
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