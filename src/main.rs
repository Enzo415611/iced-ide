use iced::{Element, Task, widget::{container, text, text_editor}};

mod ui;

fn main() -> iced::Result {
     iced::application(App::boot, App::update, App::view)
        .title("IDE")
         .run()
}


struct App{
    content: text_editor::Content,
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action)
}

impl App {
    fn boot() -> Self {
        Self {
            content: text_editor::Content::new(),
        }
    }
    
    fn update(&mut self, message: Message) ->  Task<Message> {
        match message {
            Message::Edit(action) => {
                self.content.perform(action);
                Task::none()
            }
        }
    }
}