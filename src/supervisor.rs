use std::sync::Arc;

use crate::actor::{ActorLoop, ActorProcess, Identifiable};
use crate::context::Context;
use crate::registry::Registry;


pub struct Supervisor {
    rt: Arc<tokio::runtime::Runtime>,
    registry: Arc<Registry>,
}

impl Supervisor {
    pub fn new() -> Self {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let rt_arc = Arc::new(rt);
        let registry = Arc::new(Registry::new());
        Self { rt: rt_arc, registry: registry }
    }

    pub fn run(&self) {
        self.rt.block_on(async {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        });
    }

    pub fn spawn<A: ActorProcess>(&self, actor: A)
    where
        A: ActorProcess + Send + Sync  + 'static,
    {
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        
        let mut context = Context::new(
            self.rt.clone(), 
            self.registry.clone(), 
            rx
        );
        let id: usize = context.id().clone();
        let reg = self.registry.clone();
        reg.register(id, Box::new(tx));


        self.rt.spawn(async move {
            context.run_loop(actor);
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::{echo::EchoActor, message::Message};

    use super::*;

    #[test]
    fn test_supervisor() {
        let supervisor = Supervisor::new();
        let actor = EchoActor::new(2);

        supervisor.spawn(actor);
        
        let sender = supervisor.registry.get(&2).unwrap();

        supervisor.rt.spawn(async move {
            sender.send(
                Message::Payload(2, 1)
            ).await;
        });
        
        
        supervisor.rt.block_on(async {
            let mut n = 10;
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                n -= 1;
                if n < 0 {
                    break;
                }
            }
        });
    }
}