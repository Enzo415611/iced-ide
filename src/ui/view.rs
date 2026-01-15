use iced::{Element, Length, widget::{button, column, container, text}, window};

use crate::{App, Message};

impl App {
    
    pub fn view(&self) -> Element<'_, Message> {
        
        match self.current_window {
            crate::Windows::EditorWindow => {
                let column = column![
                    self.top_bar(),
                    self.text_editor(),
                ];
                container(column).into()
            },
            crate::Windows::ConfigWindow => {
                container(text("config window")).into()
            }
        }
    }
 }