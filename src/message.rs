

pub enum Message {
    Start,
    Stop,
    Payload(usize, u8),  // (sender name, payload)
    Ping,
    Pong,
}