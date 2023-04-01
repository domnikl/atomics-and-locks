use std::sync::atomic::AtomicBool;
use std::thread;
use std::time::Duration;
use std::cell::UnsafeCell;
use std::mem::MaybeUninit;

// Mutex and Condvar can be shared between threads, so can Channel<T>
pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}

// as long as T is Send, Channel may be shared between threads safely
unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Channel<T> {
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
        }
    }

    pub unsafe fn send(&self, message: T) {
        (*self.message.get()).write(message);
        self.ready.store(true, std::sync::atomic::Ordering::Release);
    }

    pub fn is_ready(&self) -> bool {
        self.ready.load(std::sync::atomic::Ordering::Acquire)
    }

    pub unsafe fn receive(&self) -> T {
        (*self.message.get()).assume_init_read()
    } 
}

#[derive(Debug)]
enum Message {
    NewMessage(String),
}

fn main() {
    let channel: Channel<Message> = Channel::new();

    thread::scope(|s| {
        s.spawn(|| {
            let x = &channel;

            println!("Waiting for new message to process ...");

            if x.is_ready() {
                unsafe {
                    let message = x.receive();
                    println!("{:?}", message);
                }
            }
        });

        s.spawn(|| {
            let x = &channel;

            println!("Sending messages in 3 secs ...");
            thread::sleep(Duration::from_secs(3));
            unsafe {
                x.send(Message::NewMessage("Hello World".to_string()));
            }
        });
    });
}
