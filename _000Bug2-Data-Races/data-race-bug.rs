// Data Races Error
/*Data races happen when multiple threads access the same memory simultaneously with at least one write, causing unpredictable behavior.
*/
use std::sync::{Arc, Mutex};
use std::thread;
fn main(){
    // A counter variable shared by all threads
    let counter = Arc::new(Mutex::new(0));
    
    // Vector where I want to hold the handles of the spawned threads
    let mut handles = vec![];
    
    // Looping to spawn 10 threads
    for _ in 0..10{
        let counter = Arc::clone(&counter);
        
        let handle = thread::spawn(move || {
            // lock() results in lockResult contains "MutexGuard" - if lock obtained successfully.
            let mut num = counter.lock().unwrap();
            // Increment the safely accessed data
            *num +=1;
    });
    handles.push(handle);
};
    // Key concepts in preventing data races:
    // 1- Ownership
    // 2- Borrowing
    // 3- Lifetime
    // 4- Concurreny
    // Illustration of data races:
    // Thread 1-------> M1[WRITE]
    // Thread 2-------> M1[WRITE] 
    // -------------------------
    // Concurrency APIs
    
    // Blocking the main thread to wait for all threads
    for handle in handles{
        handle.join().unwrap();
    }
    
    // Printing the result stored in the Mutex
    println!("Result: {}", *counter.lock().unwrap());
    
}