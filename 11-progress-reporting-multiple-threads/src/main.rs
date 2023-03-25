use std::io::{stdout, Write};
use std::sync::atomic::{AtomicUsize};
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use std::time::Duration;

fn main() {
    let num_done = &AtomicUsize::new(0);

    thread::scope(|s| {
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    process_item(t * 25 + i);
                    num_done.fetch_add(1, Relaxed);
                }
            });
        }

        loop {
            let n = num_done.load(Relaxed);
            println!("Working ... {n}/100 done");
            thread::sleep(Duration::from_secs(1));
            if n == 100 { break; }
        }
    });

    println!("\nDone");
}

fn process_item(_i: usize) {
    let wait = rand::random::<u8>();

    thread::sleep(Duration::from_millis(wait as u64));
}
