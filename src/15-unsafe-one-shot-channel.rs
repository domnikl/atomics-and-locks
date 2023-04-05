use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::thread;

const EMPTY: u8 = 0;
const WRITING: u8 = 1;
const READY: u8 = 2;
const READING: u8 = 2;

pub struct Sender<'a, T> {
    channel: &'a Channel<T>,
}

impl<T> Sender<'_, T> {
    pub fn send(self, message: T) {
        if self
            .channel
            .state
            .compare_exchange(EMPTY, WRITING, Acquire, Relaxed)
            .is_err()
        {
            panic!("Not ready to send message");
        }

        unsafe { (*self.channel.message.get()).write(message) };
        self.channel.state.store(READY, Release);
    }
}

pub struct Receiver<'a, T> {
    channel: &'a Channel<T>,
}

impl<T> Receiver<'_, T> {
    pub fn is_ready(&self) -> bool {
        self.channel.state.load(Relaxed) == READY
    }

    pub fn receive(self) -> T {
        if self
            .channel
            .state
            .compare_exchange(READY, READING, Acquire, Relaxed)
            .is_err()
        {
            panic!("no message available");
        }

        unsafe { (*self.channel.message.get()).assume_init_read() }
    }
}

// Mutex and Condvar can be shared between threads, so can Channel<T>
pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    state: AtomicU8,
}

// as long as T is Send, Channel may be shared between threads safely
unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Channel<T> {
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            state: AtomicU8::new(EMPTY),
        }
    }

    pub fn split(&mut self) -> (Sender<T>, Receiver<T>) {
        *self = Self::new();
        (Sender { channel: self }, Receiver { channel: self })
    }
}

impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.state.get_mut() == READY {
            unsafe { self.message.get_mut().assume_init_drop() }
        }
    }
}

fn main() {
    let mut channel = Channel::new();

    thread::scope(|s| {
        let (sender, receiver) = channel.split();
        let t = thread::current();

        s.spawn(move || {
            sender.send("hello world!");
            t.unpark();
        });

        while !receiver.is_ready() {
            thread::park();
        }

        assert_eq!(receiver.receive(), "hello world!");
    });
}
