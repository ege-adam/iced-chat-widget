use std::collections::HashMap;

use rand::Rng;

use crate::message::ChatMessage;

pub struct ChatState<M>
where
    M: ChatMessage,
{
    auto_scroll: bool,
    messages: HashMap<u32, M>,
    on_message_actions: HashMap<u32, Box<dyn Fn(&M)>>,
    rng: rand::rngs::ThreadRng,
}

impl<M> ChatState<M>
where
    M: ChatMessage + Clone + 'static,
{
    pub fn new() -> Self {
        Self {
            auto_scroll: true,
            messages: HashMap::new(),
            on_message_actions: HashMap::new(),
            rng: rand::rng(),
        }
    }

    pub fn automatic_scroll(&self) -> bool {
        self.auto_scroll
    }

    pub fn set_automatic_scroll(&mut self, value: bool) {
        self.auto_scroll = value;
    }

    pub fn subscribe_on_message_action<F>(&mut self, callback: F) -> u32
    where
        F: Fn(&M) + 'static,
    {
        let id = self.rng.random::<u32>();
        self.on_message_actions.insert(id, Box::new(callback));
        id
    }

    pub fn unsubscribe_on_message_action<F>(&mut self, callback_id: &u32)
    where
        F: Fn(&M) + 'static,
    {
        self.on_message_actions.remove(callback_id);
    }

    pub fn clear_messages(&mut self) {
        self.messages.clear();
    }

    pub fn get_message(&self, message_id: &u32) -> Option<&M> {
        self.messages.get(message_id)
    }

    pub fn get_message_mut(&mut self, message_id: &u32) -> Option<&mut M> {
        self.messages.get_mut(message_id)
    }

    pub fn get_messages(&self, message_ids: Vec<&u32>) -> Vec<&M> {
        message_ids
            .iter()
            .filter_map(|id| self.messages.get(id))
            .collect()
    }

    pub fn get_messages_ids(&self) -> Vec<u32> {
        self.messages.keys().cloned().collect()
    }

    pub fn get_message_pairs(&self, message_ids: &Vec<u32>) -> Vec<(u32, &M)> {
        message_ids
            .iter()
            .filter_map(|id| self.messages.get(id).map(|msg| (*id, msg)))
            .collect()
    }

    pub fn get_all_message_pairs(&self) -> Vec<(u32, &M)> {
        self.messages.iter().map(|(id, msg)| (*id, msg)).collect()
    }

    pub fn add_messages(&mut self, messages: &Vec<M>) -> Vec<(u32, &M)> {
        let messages_internal = messages.clone();

        let new_message_ids: Vec<u32> = messages_internal
            .iter()
            .map(|message| {
                let (new_message_id, _) = self.insert_message(message.clone());
                new_message_id
            })
            .collect();
        self.get_message_pairs(&new_message_ids)
    }

    pub fn add_message(&mut self, message: M) -> (u32, &M) {
        let message = message.clone();
        self.insert_message(message)
    }

    fn insert_message(&mut self, mut message: M) -> (u32, &M) {
        let id = self.rng.random::<u32>();
        message.set_id(id);
        self.messages.insert(id, message);

        let message = &self.messages[&id];
        for callback in self.on_message_actions.values() {
            callback(message);
        }

        (id, message)
    }
}
