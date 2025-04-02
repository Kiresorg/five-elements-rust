use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let pointer_to_num_times_operated_on_by_a_thread = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let pointer_copy = Arc::clone(&pointer_to_num_times_operated_on_by_a_thread);
        let handle = thread::spawn(move || {
            let mut num = pointer_copy.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final count: {}", *pointer_to_num_times_operated_on_by_a_thread.lock().unwrap());
}
