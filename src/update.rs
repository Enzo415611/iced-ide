use std::sync::Arc;

use iced::{Task, widget::text_editor};

use crate::{App, Message, Screens, file::{open_file, read_file}};

impl App {
    pub fn update(&mut self, message: Message) ->  Task<Message> {
        match message {
            Message::ButtonTest => {
                Task::none()
            }            
            Message::Edit(action) => {
                self.content.perform(action);
                Task::none()
            },
            Message::OpenFile => {
                self.current_file_path = open_file();
                self.current_file_extension = self.current_file_path.extension().unwrap().display().to_string();
                read_file(&self.current_file_path);
                self.content.perform(text_editor::Action::Edit(text_editor::Edit::Paste(Arc::new(read_file(&self.current_file_path)))));
                Task::none()
            },
            Message::OpenConfig => {
                if self.current_window == Screens::EditorScreen {
                    self.current_window = Screens::ConfigScreen;
                } else {
                    self.current_window = Screens::EditorScreen;
                }
                Task::none()
            }
            _=> {
                Task::none()
            }
        }
    }
}