use std::{sync::{Arc, Mutex, MutexGuard}, thread::{self, JoinHandle}, rc::Rc};

fn main() {
    let counter    : Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let mut handles: Vec<thread::JoinHandle<()>> = vec![];

    for _ in 0..10 {
        let counter: Arc<Mutex<i32>> = Arc::clone(&counter);
        let handle : JoinHandle<()>  = thread::spawn(move || {
            let mut num: MutexGuard<i32> = counter.lock().unwrap();
            *num += 1
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

}
