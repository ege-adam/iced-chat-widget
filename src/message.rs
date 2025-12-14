use chrono::{DateTime, Utc};

pub trait ChatMessage {
    fn id(&self) -> u32;
    fn content(&self) -> &str;
    fn author_id(&self) -> &str;
    fn timestamp(&self) -> DateTime<Utc>; // unix timestamp
    fn is_own_message(&self) -> bool;
    fn set_id(&mut self, id: u32);
}
