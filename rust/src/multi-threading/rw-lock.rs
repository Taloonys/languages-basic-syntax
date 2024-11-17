/**
 * RwLock work: 
 * - 3 threads: 2 for read, 1 for write
 * - We need mutex-like type to allow exclusive write for single thread & and read for multiple,
 *     we need someone could decide it automaticly
 * - RwLock allow "write-thread" do his job only when all "read-thread" finished their reading
 */


use std::{sync::{Arc, Mutex, RwLock}, thread};


fn main() {
    let data = Arc::new(RwLock::new(555));                      // RwLock init, arc for MT ownershipping

    let mut t_manager = vec![];
    for _ in 1..2 {                                             // for 2 "read-threads"
        let data_clone = data.clone();                          // create new Thread-Owner for our data

        let t_read = thread::spawn(move || {
            let _data_lock = data_clone.read();                 // returns RwReadGuard
        });                                                     // RwReadGuard drop

        t_manager.push(t_read);
    }

    let _write_thread = {
        let data_clone = data.clone();                              

        let t_write = thread::spawn(move || {
            let mut data_lock = data_clone.write().unwrap();    // returns RwWriteGuard
            *data_lock += 333;                                  // exclusive change
        });                                                     // RwWriteGuard drop

        t_write.join().unwrap();
    };

    for t in t_manager {
        t.join().unwrap();
    }
}