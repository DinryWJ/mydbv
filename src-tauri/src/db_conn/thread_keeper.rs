use crossbeam::channel::{Receiver, Sender};

use crate::db_conn::{db_driver::DbDriver, message::Message};

use super::{config::DbConfig, mysql_driver::MysqlDriver};

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
        config: DbConfig,
        // f: impl FnOnce() + Send + 'static,
    ) -> Self {
        let rx_from_main_clone = rx_from_main.clone();
        let tx_to_main_clone = tx_to_main.clone();
        let f = move || {
            let driver = MysqlDriver::new(&config);
            match driver {
                Ok(d) => loop {
                    let message = rx_from_main.recv().unwrap();
                    println!("receive message: {}", message);
                    if message == "exit" {
                        break;
                    }
                    let message: Message =
                        serde_json::from_str(&message).expect("Failed to parse Message");
                    let res = d.exec_result(&message.db, &message.sql);
                    let res = serde_json::to_string(&res).unwrap();
                    tx_to_main.send(res).unwrap();
                },
                Err(e) => {
                    let message = format!("connect failed: {}", e);
                    tx_to_main.send(message).unwrap()
                }
            }
        };
        let handle = std::thread::spawn(f);
        Self {
            id,
            tx_to_subthread,
            rx_from_main: rx_from_main_clone,
            tx_to_main: tx_to_main_clone,
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

    pub fn destory(&self) -> Result<String, crossbeam::channel::RecvError> {
        self.tx_to_subthread.send("exit".to_string()).unwrap();
        self.rx_from_subthread.recv()
    }
}
