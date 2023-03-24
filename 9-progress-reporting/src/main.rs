use std::collections::VecDeque;
use std::io::{stdout, Write};
use std::sync::{Condvar, Mutex};
use std::sync::atomic::{AtomicBool, AtomicUsize};
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use std::time::Duration;

fn main() {
    let num_done = AtomicUsize::new(0);

    // takes care of joining all the threads
    thread::scope(|s| {
        // a background thread to process all 100 items

        s.spawn(|| {
            for i in 0..100 {
                process_item(i);
                num_done.store(i + 1, Relaxed);
            }
        });

        loop {
            let n = num_done.load(Relaxed);
            print!("\rWorking ... {n}/100 done");
            stdout().flush().unwrap();
            if n == 100 { break; }

            thread::sleep(Duration::from_millis(100));
        }
    });

    println!("\nDone");
}

fn process_item(i: usize) {
    let wait = rand::random::<u8>();

    thread::sleep(Duration::from_millis(wait as u64));
}
