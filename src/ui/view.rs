use iced::{Element, Length, widget::{column, container, text}};

use crate::{App, Message};

impl App {
    
    pub fn view(&self) -> Element<'_, Message> {
        
        let column = column![
            self.text_editor()
        ];
        container(column).into()
    }
 }