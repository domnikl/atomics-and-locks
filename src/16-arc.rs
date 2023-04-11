use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicUsize, fence};
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};

struct ArcData<T> {
    ref_count: AtomicUsize,
    data: T,
}

pub struct Arc<T> {
    ptr: NonNull<ArcData<T>>,
}

unsafe impl<T: Send + Sync> Send for Arc<T> {}

unsafe impl<T: Send + Sync> Sync for Arc<T> {}

impl<T> Arc<T> {
    pub fn new(data: T) -> Arc<T> {
        Arc {
            ptr: NonNull::from(Box::leak(Box::new(ArcData {
                ref_count: AtomicUsize::new(1),
                data,
            }))),
        }

        // compiler doesn't know that it will always point to a valid ArcData<T>
    }

    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }

    pub fn get_mut(arc: &mut Self) -> Option<&mut T> {
        if arc.data().ref_count.load(Relaxed) == 1 {
            fence(Acquire);
            // Safety: Nothing else can access the data, since there's
            // only one Arc to which we have exclusive access
            unsafe { Some(&mut arc.ptr.as_mut().data) }
        } else {
            None
        }
    }
}

impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data().data
    }
}

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        // TODO: handle overflows
        self.data().ref_count.fetch_add(1, Relaxed);

        Arc { ptr: self.ptr }
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        // to ensure that there is a happens-before relationship with every previous fetch_sub operation,
        // we can do using release and acquire ordering
        if self.data().ref_count.fetch_sub(1, Release) == 1 {
            unsafe {
                drop(Box::from_raw(self.ptr.as_ptr()));
            }
        }
    }
}

#[test]
fn test() {
    static NUM_DROPS: AtomicUsize = AtomicUsize::new(0);

    struct DetectDrop;

    impl Drop for DetectDrop {
        fn drop(&mut self) {
            NUM_DROPS.fetch_add(1, Relaxed);
        }
    }

    // Create two Arcs sharing an object containing a string and a DetectDrop
    let x = Arc::new(("hello", DetectDrop));
    let y = x.clone();

    // Send it to another thread, and use it there.
    let t = std::thread::spawn(move || {
        assert_eq!(x.0, "hello");
    });

    // in parallel, y should still be usable here
    assert_eq!(y.0, "hello");

    // wait for the thread to finish
    t.join().unwrap();

    // One Arc, x, should be dropped by now
    // we still have y, so the object shouldn't have been dropped yet.
    assert_eq!(NUM_DROPS.load(Relaxed), 0);

    drop(y);

    // Now that y is dropped the object should've been dropped
    assert_eq!(NUM_DROPS.load(Relaxed), 1);
}

fn main() {}

