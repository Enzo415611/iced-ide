use std::fmt::Debug;

use iced::{Task, widget::{text_editor}, window::{self, Id}};

mod ui;

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
}


#[derive(Debug, Clone)]
enum Message {
    ButtonTest,
    CloseWindow,
    Edit(text_editor::Action),
    SaveFile   
}


impl App {
    fn boot() -> Self {
        Self {
            content: text_editor::Content::new(),
            current_window: Windows::EditorWindow
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
            _=> {
                Task::none()
            }
            
            
        }
    }
}