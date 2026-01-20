use iced::{Color, Element, Length, border::Radius, widget::text_editor};

use crate::{App, Message, ui::colors::{BORDER_COLOR}};

impl App {
    pub fn text_editor(&self) -> Element<'_, Message> {
        text_editor(&self.content)
            .on_action(Message::Edit)
            .height(Length::Fill)
            .highlight(&self.current_file_extension, iced::highlighter::Theme::SolarizedDark)
            .wrapping(iced::widget::text::Wrapping::None)
            .placeholder("Editor")
            .style(|_, _| text_editor_style())
            .into()
    }
}


fn text_editor_style() -> text_editor::Style {
    text_editor::Style {
        background: iced::Background::Color(Color::TRANSPARENT),
        border: iced::Border { color: BORDER_COLOR, width: 0.0, radius: Radius::new(0.0) },
        placeholder: Color::WHITE,
        selection: Color { r: 0.0, g: 173.0, b: 245.0, a: 0.8 },
        value: Color::WHITE
    }
}