use std::sync::Mutex;
use std::thread;
use std::sync::Condvar;
use std::collections::VecDeque;
use std::time::Duration;

// Mutex and Condvar can be shared between threads, so can Channel<T>
pub struct Channel<T> {
    queue: Mutex<VecDeque<T>>,
    item_ready: Condvar,
}

impl<T> Channel<T> {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            item_ready: Condvar::new(),
        }
    }

    pub fn send(&self, message: T) {
        self.queue.lock().unwrap().push_back(message);
        self.item_ready.notify_one();
    }

    pub fn receive(&self) -> T {
        let mut b = self.queue.lock().unwrap();

        loop {
            if let Some(message) = b.pop_front() {
                return message;
            }

            // wait() will unlock the Mutex and lock again if it returns to
            // not keep the lock while waiting
            b = self.item_ready.wait(b).unwrap();
        }
    }
}

enum Message {
    NewMessage(String),
    Terminate,
}


fn main() {
    let channel: Channel<Message> = Channel::new();

    thread::scope(|s| {
        s.spawn(|| {
            let x = &channel;

            loop {
                println!("Waiting for new message to process ...");
                match x.receive() {
                    Message::NewMessage(message) => println!("{}", message),
                    Message::Terminate => break,
                };
            };
        });

        s.spawn(|| {
            let x = &channel;

            println!("Sending messages now ...");
            x.send(Message::NewMessage("Hello World".to_string()));
            thread::sleep(Duration::from_secs(3));
            x.send(Message::Terminate);
        });
    });
}

