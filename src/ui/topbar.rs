use iced::{
    Alignment, Color, Element, Length::{self}, border::Radius, widget::{self, button, image, row, text}
};
use iced_aw::{Menu, MenuBar, menu::Item};

use crate::{App, Message, ui::colors::{BORDER_COLOR, BUTTON_BACKGROUND_COLOR, BUTTON_HOVERED_COLOR}};




impl App {
    
    pub fn top_bar(&self) -> Element<'_, Message> {
        let file_menu = Item::with_menu(
            top_bar_menu_button(Message::ButtonTest, "File".to_string()),
            Menu::new(
                vec![
                    Item::new(top_bar_menu_button(Message::OpenFile, "Open File".to_string())),
                    Item::new(top_bar_menu_button(Message::SaveFile, "Save".to_string()))                  
                ])
                .spacing(2.0)
                .max_width(100.0),
        );

        let open_menu = Item::with_menu(
            button(image(self.images.get(0).unwrap()))
                .style(|_, s| button_menu_style(&s))
                .on_press(Message::ButtonTest),
            Menu::new(vec![file_menu]).max_width(100.0),
        );

        row![
            row![
                MenuBar::new(vec![open_menu]),
            ]
            .width(Length::Fill)
            .align_y(Alignment::Start),
            row![
                button(image(self.images.get(1).unwrap())).style(|_,s| button_menu_style(&s) ).on_press(Message::OpenConfig)
            ]
            .spacing(1.0)
            .align_y(Alignment::End)
        ]
        .height(20.0)
        .into()
    }
}


fn top_bar_menu_button(
    message: Message,
    content: String,
) -> Element<'static, Message> {
    button(
        text(content)
            .center()
            .wrapping(text::Wrapping::None)
            .size(15.0)
    ).on_press(message)
    .width(Length::Fill)
    .height(20.0)
    .style(move |_, s| button_menu_style(&s))
    .into()
    
}

fn button_menu_style(s: &button::Status) -> widget::button::Style {
    match s {
        button::Status::Hovered => {
            button::Style {
                text_color: Color::WHITE,
                border: iced::Border {
                    color: BORDER_COLOR,
                    width: 0.8,
                    radius: Radius::new(10.0),
                },
                background: Some(iced::Background::Color(BUTTON_HOVERED_COLOR)),
                ..Default::default()
            }
        },
        _ => {
            button::Style {
                text_color: Color::BLACK,
                border: iced::Border {
                    color: BORDER_COLOR,
                    width: 0.8,
                    radius: Radius::new(10.0),
                },
                background: Some(iced::Background::Color(BUTTON_BACKGROUND_COLOR)),
                ..Default::default()
            }
        }
    }
    
}