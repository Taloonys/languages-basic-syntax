/**
 * Barriers are like `WaitGroup` in Golang. 
 * We can block any thread in purpose to wait for some threads to finish their jobs.
 */


use std::sync::{Arc, Barrier};
use std::thread;
 

fn main() {
    let wait_number_of_threads = 3;
    let barrier = Arc::new(Barrier::new(wait_number_of_threads));   // We will wait for 3 threads to finish!

    let mut t_manager = vec![]; 
    for _ in 0..3 {
        let barrier_clone = barrier.clone();

        let new_thread = thread::spawn(move || {
            thread::sleep(std::time::Duration::from_secs(2));       // *dummy code*
            barrier_clone.wait();                                   // more like: We wait untill 3 threads reach this point
        });
         
        t_manager.push(new_thread);
    }
 
    for t in t_manager { 
        t.join().unwrap();
    }
}