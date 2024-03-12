use crossbeam::channel::{Receiver, Sender};
use serde::{Deserialize, Serialize};

pub struct ThreadKeeper {
    pub id: u32,
    pub tx_to_subthread: Sender<String>,
    pub rx_from_main: Receiver<String>,
    pub tx_to_main: Sender<String>,
    pub rx_from_subthread: Receiver<String>,
    pub thread: std::thread::JoinHandle<()>,
}

impl ThreadKeeper {
    pub fn new(
        id: u32,
        tx_to_subthread: Sender<String>,
        rx_from_main: Receiver<String>,
        tx_to_main: Sender<String>,
        rx_from_subthread: Receiver<String>,
        f: impl FnOnce() + Send + 'static,
    ) -> Self {
        let handle = std::thread::spawn(f);
        Self {
            id,
            tx_to_subthread,
            rx_from_main,
            tx_to_main,
            rx_from_subthread,
            thread: handle,
        }
    }

    pub fn send_message(&self, message: Message) -> Result<String, crossbeam::channel::RecvError> {
        self.tx_to_subthread
            .send(serde_json::to_string(&message).unwrap())
            .unwrap();
        self.rx_from_subthread.recv()
    }

}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub db: String,
    pub sql: String,
}
