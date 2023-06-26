use std::{collections::HashMap, ops::{DerefMut, Deref}, sync::{Arc, Mutex}};

use tokio::sync::mpsc;

use crate::message::Message;


pub struct Registry {
    data: Arc<Mutex<HashMap<usize, Box<mpsc::Sender<Message>>>>>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register(&self, addr: usize, sender: Box<mpsc::Sender<Message>>){
        let mut data = self.data.lock().unwrap();
        data.insert(addr, sender);
    }

    pub fn get(&self, addr: &usize) -> Option<Box<mpsc::Sender<Message>>> {
        let mut data = self.data.lock().unwrap();
        let sender = data.get(addr).cloned();
        sender
    }
}
