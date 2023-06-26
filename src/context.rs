use std::{sync::{Arc, atomic::{AtomicUsize, Ordering}}};

use tokio::{runtime::Runtime, sync::mpsc};

use crate::{registry::Registry, message::{Message}, actor::{ActorProcess, ActorLoop, Identifiable}};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct Context<T: ActorProcess> {
    id: usize,
    rt: Arc<Runtime>,
    registry: Arc<Registry>,
    receiver: mpsc::Receiver<Message>,

    phantom: std::marker::PhantomData<T>,
}

impl<T: ActorProcess> Context<T> {
    pub fn new(rt: Arc<Runtime>, registry: Arc<Registry>, receiver: mpsc::Receiver<Message>) -> Self {
        Self {
            id: COUNTER.fetch_add(1, Ordering::SeqCst),
            rt: rt,
            registry: registry,
            receiver: receiver,

            phantom: std::marker::PhantomData,
        }
    }
}

impl<T> Identifiable for Context<T> where T: ActorProcess + Send + Sync + 'static {
    fn id(&self) -> usize {
        self.id
    }
}

impl<T> ActorLoop<T> for Context<T> where T: ActorProcess + Send + Sync + 'static {
    fn run_loop(&mut self, actor: T) {
        loop {
            match self.receiver.try_recv() {
                Ok(message) => actor.process(self, message),
                Err(_) => todo!(),
            }
        }
    }
}