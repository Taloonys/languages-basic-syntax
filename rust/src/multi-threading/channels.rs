/**
 * Channels - are like single write-read data-pipes for multithreading
 */


use std::thread;
use std::sync::mpsc;
 

fn main() {
    let (sender, receiver) = mpsc::channel();               // channel init
    
    let sender_clone = sender.clone();                      // in purpose to move sender into thread, we can have multiple senders
    let worker = thread::spawn(move || {
        sender_clone.send(555).unwrap();                    // send to channel
    });

    let received_data = receiver.recv().unwrap();           // read from channel
    println!("Got msg: {}", received_data);
 
    worker.join().unwrap();
}