# Iced Chat Widget

A customizable chat widget for the Iced GUI library. This crate provides a ready-to-use widget for displaying chat messages with support for custom themes, message actions, and varied message alignments.

## Features

- **Customizable Styling**: Fully configurable themes for background, text, and message bubbles.
- **Message Actions**: Interactive buttons attached to messages for easy user interaction.
- **Automatic Sorting**: Messages are automatically sorted by timestamp.
- **Flexible Integration**: Easy to integrate into existing Iced applications using the `ChatMessage` trait.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
iced-chat-widget = "0.1.0"
```

## Usage

To use the widget, implement the `ChatMessage` trait for your message struct and manage the `ChatState`.

```rust
use iced::{Element, Theme};
use iced_chat_widget::{ChatWidget, ChatState, ChatMessage};
use chrono::{DateTime, Utc};

#[derive(Clone, Debug)]
struct MyMessage {
    id: u32,
    content: String,
    author_id: String,
    timestamp: DateTime<Utc>,
    is_me: bool,
}

impl ChatMessage for MyMessage {
    fn id(&self) -> u32 {
        self.id
    }
    
    fn content(&self) -> &str {
        &self.content
    }
    
    fn author_id(&self) -> &str {
        &self.author_id
    }
    
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
    
    fn is_own_message(&self) -> bool {
        self.is_me
    }
    
    fn set_id(&mut self, id: u32) {
        self.id = id;
    }
}

struct ExampleApp {
    state: ChatState<MyMessage>,
}

impl ExampleApp {
    fn view(&self) -> Element<iced_chat_widget::action::ChatEvent> {
        // Initialize widget with the current theme and chat state
        ChatWidget::new(&Theme::Dark, &self.state).view()
    }
}
```

## License

This project is licensed under the GPL-2.0 License.
