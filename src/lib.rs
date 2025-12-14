pub mod action;
mod consts;
pub mod message;
pub mod state;
pub mod style;

use crate::{
    action::{ChatEvent, MessageAction},
    consts::*,
    message::ChatMessage,
    state::ChatState,
    style::ChatTheme,
};

use iced::{
    Alignment::{self},
    Element,
    Length::{self, Fill},
    Task, Theme,
    alignment::Horizontal::Right,
    widget::{button, column, container, row, scrollable, text},
};

pub struct ChatWidget<'a, M>
where
    M: ChatMessage,
{
    state: &'a ChatState<M>,
    actions: Vec<MessageAction>,
    theme: ChatTheme,
}

impl<'a, M> ChatWidget<'a, M>
where
    M: ChatMessage + Clone + 'static,
{
    pub fn new(theme: &Theme, state: &'a ChatState<M>) -> Self {
        Self {
            state: state,
            actions: Vec::new(),
            theme: ChatTheme::get_default(theme),
        }
    }

    pub fn with_custom_theme(mut self, theme: ChatTheme) -> Self {
        self.theme = theme;
        self
    }

    pub fn with_actions(mut self, actions: Vec<MessageAction>) -> Self {
        self.actions = actions;
        self
    }

    pub fn scroll_to_bottom() -> Task<ChatEvent> {
        scrollable::snap_to(
            scrollable::Id::new(SCROLLABLE_ID),
            scrollable::RelativeOffset::END,
        )
    }

    pub fn view(self) -> Element<'static, ChatEvent> {
        let theme = self.theme;
        let actions = self.actions;

        let ids = self.state.get_messages_ids();

        let mut messages_ordered = ids
            .iter()
            .filter_map(|id| self.state.get_message(id))
            .collect::<Vec<&M>>();
        messages_ordered.sort_by_key(|msg| msg.timestamp());

        let messages_ordered = messages_ordered
            .iter()
            .fold(column![].spacing(theme.spacing()), |col, msg| {
                col.push(Self::render_message_owned(msg, &theme, actions.clone()))
            });

        let scrollable_messages = scrollable(
            container(messages_ordered)
                .padding(theme.padding())
                .width(Length::Fill),
        )
        .height(Length::Fill);

        let scrollable_messages = scrollable_messages.id(scrollable::Id::new(SCROLLABLE_ID));

        let bg_color = theme.background_color();
        let widget = container(scrollable_messages)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_theme| container::Style {
                background: Some(bg_color.into()),
                ..Default::default()
            })
            .into();

        widget
    }

    fn render_message_owned(
        msg: &M,
        theme: &ChatTheme,
        actions: Vec<MessageAction>, // Takes owned Vec, not a slice
    ) -> Element<'static, ChatEvent> {
        let style = if msg.is_own_message() {
            theme.own_message_style()
        } else {
            theme.other_message_style()
        };

        let is_own = msg.is_own_message();
        let msg_id = msg.id().to_string();
        let author = msg.author_id().to_string();
        let content = msg.content().to_string();

        // Extract all values from style upfront (copy them)
        let author_text_size = style.author_text_size();
        let content_text_size = style.content_text_size();
        let text_color = style.text_color();
        let padding = style.padding();
        let background = style.background();
        let border_radius = style.border_radius();

        let mut message_col = column![]
            .spacing(4.0)
            .push(text(author).size(author_text_size).color(text_color))
            .push(text(content).size(content_text_size).color(text_color));

        if !actions.is_empty() {
            let actions_row = actions
                .into_iter() // Consume the owned Vec
                .fold(row![].spacing(5.0), |row_acc, action| {
                    let action_id = action.id; // Move, not clone
                    let label = action.label; // Move, not clone
                    let msg_id_clone = msg_id.clone();
                    row_acc.push(
                        button(text(label).size(12))
                            .on_press(ChatEvent::ActionClicked {
                                message_id: msg_id_clone,
                                action_id,
                            })
                            .padding(5),
                    )
                });
            message_col = message_col.push(actions_row);
        }

        let timestamp_text_size = style.timestamp_text_size();
        let timestamp = msg.timestamp();
        let timestamp_text = format!(
            "{}",
            timestamp.format(style.time_stamp_format()).to_string()
        );
        let timestamp_container = container(
            text(timestamp_text)
                .size(timestamp_text_size)
                .color(style.time_stamp_text_color()),
        )
        .align_x(Right);

        let timestamp_row = row![timestamp_container];
        message_col = message_col.push(timestamp_row);

        let message_container = container(message_col)
            .padding(padding)
            .style(move |_theme| container::Style {
                background: Some(iced::Background::Color(background)),
                border: iced::Border {
                    radius: border_radius,
                    ..Default::default()
                },
                ..Default::default()
            });

        let message_wrapper = container(message_container).width(Fill).align_x(if is_own {
            Right
        } else {
            iced::alignment::Horizontal::Left
        });

        let row_widget: iced::widget::Row<'static, ChatEvent> = row![message_wrapper].width(Fill);

        if is_own {
            row_widget.align_y(Alignment::End).into()
        } else {
            row_widget.align_y(Alignment::Start).into()
        }
    }
}
