/**
 * Rust imply pointers, static vars and unions like `unsafe` block, and you have to declare it.
 * Unsafe context still could be checked for by rust analyzer, borrow checker and etc.
 */


static mut NUMBER: i32 = 0;                         // it's ok to just exist


unsafe fn unsafe_func() {                           // unsafe function
    let a   = 5;
    let pa  = &a as *const i32;

    unsafe {
        let _npa = *pa;
    }
}


union Symbol {
    letter  : char,
    code    : u32,
}


fn main() {
    unsafe {
        NUMBER += 5;                                // changing static value isn't ok
        println!("{NUMBER}");                       // get static value is also unsafe
    }

    // unsafe_func();                               /* <-- Err */
    unsafe {
        unsafe_func()                               // unsafe func calls only inside unsafe blocks
    }

    let _unions = {
        let l   = Symbol{ letter: 'A' };
        let c   = Symbol{ code: 65 };

        unsafe {
            println!("{}, {}", c.letter, l.code);
        }
    };
}