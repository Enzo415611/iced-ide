use iced::{
    Element, Length, widget::{column, container, pane_grid, row, text}
};

use crate::{App, Message, Pane, ui::colors::BORDER_COLOR};

impl App {
    pub fn view(&self) -> Element<'_, Message> {
        column![
            row![
                self.top_bar()
            ],
            container("")
                .width(Length::Fill)
                .height(0.8)
                .style(|_| {
                    container::Style {
                        background: Some(iced::Background::Color(BORDER_COLOR)),
                        ..Default::default()
                    }
                })                
            ,
            match self.current_window {
                crate::Screens::EditorScreen => {
                    let pane_grid = pane_grid(&self.panes, |_, state, _| {
                        match state {
                            Pane::EditorPane => {
                                container(self.text_editor()).into()
                            },
                            Pane::TreeViewPane => {
                                container(text("TreeViewPane")).into()
                            }
                        }
                    })
                    .on_resize(10.0, Message::PaneResized);
                    
                    let column = column![                        
                        pane_grid
                    ];
                    container(column)
                }
                crate::Screens::ConfigScreen => container(text("config window")).into(),
            }
        ].spacing(1.0)
        .into()
        
    }
}
