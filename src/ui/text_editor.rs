use iced::{Element, Length, widget::text_editor};

use crate::{App, Message};

impl App {
    pub fn text_editor(&self) -> Element<'_, Message> {
        text_editor(&self.content)
            .on_action(Message::Edit)
            .height(Length::Fill)
            .into()
    }
}