use iced::{
    Alignment, Color, Element, Length::{self}, border::Radius, widget::{self, button, image, row, text}
};
use iced_aw::{Menu, MenuBar, menu::Item};

use crate::{App, Message, ui::colors::BORDER_COLOR};

impl App {
    pub fn top_bar(&self) -> Element<'_, Message> {
        let file_menu = Item::with_menu(
            top_bar_button(),
            Menu::new(
                vec![
                    Item::new(top_bar_menu_button(Message::ButtonTest, "Open File".to_string(), button_menu_style())),
                    Item::new(top_bar_menu_button(Message::SaveFile, "Save".to_string(), button_menu_style()))
                ])
            .spacing(2.0)
            .max_width(100.0),
        );

        let open_menu = Item::with_menu(
            button(image("src/assets/menu-hamburguer.png")
                
            ).style(|_, _| {
                button::Style {
                    ..Default::default()
                }
            }),
            Menu::new(vec![file_menu]).max_width(100.0),
        );

        row![
            row![
                MenuBar::new(vec![open_menu])
            ].width(Length::Fill)
            .align_y(Alignment::Start),
            row![
                button(text("X").size(10.0)).height(Length::Fill).on_press(Message::CloseWindow)
            ]
            .align_y(Alignment::End)
        ].height(20.0)
        .into()
    }
}

fn top_bar_button() -> Element<'static, Message> {
    button(text("File").center().size(15.0))
        .on_press(Message::ButtonTest)
        .style(|_, _| top_bar_button_style())
        .height(20.0)
        .width(Length::Fill)
        .into()
}

fn top_bar_button_style() -> button::Style {
    button::Style {
        snap: true,
        border: iced::Border {
            color: BORDER_COLOR,
            width: 0.8,
            radius: Radius::new(0),
        },
        text_color: Color::WHITE,
        ..Default::default()
    }
}

fn top_bar_menu_button(
    message: Message,
    content: String,
    style: button::Style,
) -> Element<'static, Message> {
    button(text(content).center().size(15.0))
        .on_press(message)
        .width(Length::Fill)
        .height(20.0)
        .style(move |_, _| style)
        .into()
}

fn button_menu_style() -> widget::button::Style {
    button::Style {
        text_color: Color::WHITE,
        border: iced::Border {
            color: BORDER_COLOR,
            width: 0.8,
            radius: Radius::new(0),
        },
        ..Default::default()
    }
}
