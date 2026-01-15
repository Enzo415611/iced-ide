use std::{fmt::Debug, path::PathBuf, sync::Arc};

use iced::{Task, widget::{text_editor}};

use crate::file::{open_file, read_file};

mod ui;
mod file;

fn main() -> iced::Result {
     iced::application(App::boot, App::update, App::view)
        .title("IDE")
        .settings(iced::Settings {
            id: Some("IDE".to_string()),
            ..Default::default() 
        })
        .decorations(false)
        .run()
}

enum Windows {
    EditorWindow,
    ConfigWindow,
}

struct App{
    current_window: Windows,
    content: text_editor::Content,
    current_file_path: PathBuf,
    current_file_extension: String,
}


#[derive(Debug, Clone)]
enum Message {
    ButtonTest,
    OpenFile,
    CloseWindow,
    Edit(text_editor::Action),
    SaveFile   
}


impl App {
    fn boot() -> Self {
        Self {
            content: text_editor::Content::new(),
            current_window: Windows::EditorWindow,
            current_file_path: PathBuf::new(),
            current_file_extension: String::new()  
        }
    }
    
    fn update(&mut self, message: Message) ->  Task<Message> {
        match message {
            Message::ButtonTest => {
                Task::none()
            }
            Message::CloseWindow => {
                iced::exit()
            },
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
            }
            _=> {
                Task::none()
            }
            
            
        }
    }
}