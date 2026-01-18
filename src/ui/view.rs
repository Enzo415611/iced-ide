use iced::{
    Element,
    widget::{column, container, row, text},
};

use crate::{App, Message};

impl App {
    pub fn view(&self) -> Element<'_, Message> {
        column![
            row![self.top_bar()],
            match self.current_window {
                crate::Screens::EditorScreen => {
                    let column = column![
                        self.text_editor(),
                    ];
                    container(column)
                }
                crate::Screens::ConfigScreen => container(text("config window")).into(),
            }
        ].into()
        
    }
}
