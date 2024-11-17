/**
 * TLS - Thread Local Storage -> obv, local data for current thread
 */


use std::thread;
use std::cell::RefCell;


thread_local! {
    static TLS_DATA: RefCell<i32> = RefCell::new(555);
}


fn main() {
    let handles: Vec<_> = (0..5).map(|i| {                          // apply to vector `0 1 2 3 4` lambda
        thread::spawn(move || {                                     
            TLS_DATA.with(|data| {                                  // apply lambda where data is our `RefCell`
                let mut value = data.borrow_mut();
                *value += i;                                        // change tls data
                println!("Thread {}: TLS value = {}", i, *value);
            });
        })
    }).collect();                                                   // "finish vector build"

    for handle in handles {
        handle.join().unwrap();
    }
}