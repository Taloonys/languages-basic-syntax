use std::{sync::{Arc, Mutex}, thread};


fn main() {
    let data = Arc::new(Mutex::new(555));                       // Mutex init, Arc for thread ownershipping on signle resource

    let mut t_manager = vec![];

    for _ in 1..2 {
        let data_clone = data.clone();                          // create new Thread-Owner for our data

        let t = thread::spawn(move || {                         // move our data owner to the thread
            let mut data_lock = data_clone.lock().unwrap();     // mutex lock (returns Result<>), unwrap returns `MutexGuard`
            *data_lock += 100;                                  // exclusive data change
        });                                                     // here MutexGuard will unlock itself automaticly

        t_manager.push(t);
    }

    for t in t_manager {
        t.join().unwrap();
    }
}