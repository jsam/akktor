use crate::{actor::{ActorProcess, Identifiable}, message::Message, context::Context};



pub struct EchoActor {
    id: usize,
}

impl EchoActor {
    pub fn new(addr: usize) -> Self {
        Self {
            id: addr,
        }
    }
}

impl ActorProcess for EchoActor {
    fn process(&self, ctx: &mut Context<Self>, message: Message) {
        println!("EchoActor: process");
    }
}