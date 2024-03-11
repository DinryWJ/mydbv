use crossbeam::channel::{Receiver, Sender};
use serde::{Deserialize, Serialize};

pub struct ThreadKeeper {
    pub id: u32,
    pub s: Sender<String>,
    pub r: Receiver<String>,
    pub thread: std::thread::JoinHandle<()>,
}

impl ThreadKeeper {
    pub fn new(
        id: u32,
        s: Sender<String>,
        r: Receiver<String>,
        f: impl FnOnce() + Send + 'static,
    ) -> Self {
        let handle = std::thread::spawn(f);
        Self {
            id,
            s,
            r,
            thread: handle,
        }
    }

    pub fn send_message(&self, message: Message) -> Result<String, crossbeam::channel::RecvError> {
        self.s
            .send(serde_json::to_string(&message).unwrap())
            .unwrap();
        self.r.recv()
    }

    pub fn receive_message(&mut self) -> Option<String> {
        self.r.recv().ok()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub db: String,
    pub sql: String,
}
