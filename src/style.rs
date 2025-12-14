use iced::Color;
use iced::Theme;
use iced::border::Radius;

#[derive(Debug, Clone)]
pub struct ChatTheme {
    own_message_style: MessageStyle,
    other_message_style: MessageStyle,
    background_color: Color,
    spacing: f32,
    padding: f32,
}

#[derive(Debug, Clone)]
pub struct MessageStyle {
    background: Color,
    text_color: Color,
    border_radius: Radius,
    padding: f32,
    author_text_size: f32,
    content_text_size: f32,
    time_stamp_text_color: Color,
    timestamp_text_size: f32,
    time_stamp_format: String,
}

impl MessageStyle {
    pub fn background(&self) -> Color {
        self.background
    }

    pub fn text_color(&self) -> Color {
        self.text_color
    }

    pub fn time_stamp_text_color(&self) -> Color {
        self.time_stamp_text_color
    }

    pub fn border_radius(&self) -> Radius {
        self.border_radius
    }

    pub fn padding(&self) -> f32 {
        self.padding
    }

    pub fn author_text_size(&self) -> f32 {
        self.author_text_size
    }

    pub fn content_text_size(&self) -> f32 {
        self.content_text_size
    }

    pub fn timestamp_text_size(&self) -> f32 {
        self.timestamp_text_size
    }

    pub fn time_stamp_format(&self) -> &str {
        &self.time_stamp_format
    }
}

impl ChatTheme {
    pub fn get_default(iced_theme: &Theme) -> Self {
        // Define default styles
        let own_message_style = MessageStyle {
            background: iced_theme.extended_palette().background.strong.color,
            text_color: iced_theme.extended_palette().background.strong.text,
            time_stamp_text_color: iced_theme.extended_palette().background.strong.text,
            border_radius: Radius::new(8),
            padding: 10.0,
            author_text_size: 14.0,
            content_text_size: 16.0,
            timestamp_text_size: 10.0,
            time_stamp_format: String::from("%H:%M:%S"),
        };

        let other_message_style = MessageStyle {
            background: iced_theme.extended_palette().background.weak.color,
            text_color: iced_theme.extended_palette().background.weak.text,
            time_stamp_text_color: iced_theme.extended_palette().background.weak.text,
            border_radius: Radius::new(8),
            padding: 10.0,
            author_text_size: 14.0,
            content_text_size: 16.0,
            timestamp_text_size: 10.0,
            time_stamp_format: String::from("%H:%M:%S"),
        };

        ChatTheme {
            own_message_style,
            other_message_style,
            background_color: iced_theme.extended_palette().background.base.color,
            spacing: 10.0,
            padding: 10.0,
        }
    }

    pub fn own_message_style(&self) -> &MessageStyle {
        &self.own_message_style
    }

    pub fn other_message_style(&self) -> &MessageStyle {
        &self.other_message_style
    }

    pub fn background_color(&self) -> Color {
        self.background_color
    }

    pub fn spacing(&self) -> f32 {
        self.spacing
    }

    pub fn padding(&self) -> f32 {
        self.padding
    }
}
