use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
 

fn main() {
    let data = Arc::new(AtomicUsize::new(0));                       // Arc for creating MT ownershipping

    let mut t_manager = vec![]; 
    for _ in 0..3 {
        let data_clone = data.clone();

        let new_thread = thread::spawn(move || {
            data_clone.fetch_add(1, Ordering::Relaxed);             // memory ordering is also supported
        });
         
        t_manager.push(new_thread);
    }
 
    for t in t_manager { 
        t.join().unwrap();
    }
 
    let _value = data.load(Ordering::Relaxed);
}