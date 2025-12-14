#[derive(Debug, Clone)]
pub struct MessageAction {
    pub id: String,
    pub label: String,
    pub icon: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ChatEvent {
    ActionClicked {
        message_id: String,
        action_id: String,
    },
    MessageClicked {
        message_id: String,
    },
    ParticipantClicked {
        participant_id: String,
    },
}
