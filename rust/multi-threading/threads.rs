/**
 * Rust also has coroutines but there are different implementations for this, like "may" crate, Rfc2033, "tokio" and etc
 */


use std::{sync::Arc, thread};


fn main() {
    let t = thread::spawn(|| { 
        println!("weird attempt") 
    });

    let sum = |x, y| { x + y };
    let t1 = thread::spawn(move || {        // pass ownershipping to t1 thread context
        println!("1 + 3 = {}", sum(1, 3))
    });

    t.join().unwrap();                      // `join()` returns `Result<>`
    t1.join().unwrap();

    let _arc_pointer = {                    // Atomic Reference Counting ~ thread safe version of `Rc`
        let i32_heap = Arc::new(555);
        let _i32_heap1 = i32_heap.clone();

        /**
         * Arc contains 2 counters: Ref counter on object and thread counter.
         * Arc contains `ArcInner` (smart pointer) that uses RwLock (mt-sync tool)
         */
    };
}