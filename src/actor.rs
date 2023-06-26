
use crate::context::Context;
use crate::message::Message;

pub trait Identifiable {
    fn id(&self) -> usize;
}

pub trait ActorProcess: Sized {
    fn process(&self, ctx: &mut Context<Self>, message: Message);
}

pub trait ActorLoop<T>
where 
    T: ActorProcess,
    Self: Identifiable + Send + Sync + 'static 
{   

    fn run_loop(&mut self, process: T);
}
