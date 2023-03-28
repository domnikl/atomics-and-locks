use std::rc::Rc;
use std::sync::Arc;
use std::thread;

fn main() {
    let a = Rc::new([1, 2, 3]);
    let b = a.clone();

    assert_eq!(a.as_ptr(), b.as_ptr()); // Same allocation!

    // thread::spawn(move || dbg!(b)); // ERROR: cannot be sent between threads safely!

    let c = Arc::new([1, 2, 3]);
    let d = c.clone();

    thread::spawn(move || dbg!(d)).join().unwrap();
}
