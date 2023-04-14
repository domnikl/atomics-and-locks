use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Counter protected by a mutex
    let counter = Arc::new(Mutex::new(0));

    let t1 = {
        let counter = counter.clone();

        thread::spawn(move || {
            let mut value = counter.lock().unwrap();
            *value += 5;
        })
    };

    let t2 = {
        let counter = counter.clone();

        thread::spawn(move || {
            let mut value = counter.lock().unwrap();
            *value += 2;
        })
    };

    t1.join().unwrap();
    t2.join().unwrap();

    assert_eq!(*counter.lock().unwrap(), 7);
}
