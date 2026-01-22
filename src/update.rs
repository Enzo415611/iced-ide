use std::{fs, sync::Arc};

use iced::{Task, keyboard::{Key, Modifiers}, widget::{pane_grid::{self}, text_editor}};

use crate::{App, Message, Screens, file::{open_file, read_file, save_file}};

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
                if let Some(extension) = self.current_file_path.extension() {
                    self.current_file_extension = extension.display().to_string();
                }
                self.content.perform(text_editor::Action::Edit(text_editor::Edit::Paste(Arc::new(read_file(&self.current_file_path)))));
                Task::none()
            },
            Message::SaveFile => {
                if self.current_file_path.exists() {
                    _=fs::write(&self.current_file_path, self.content.text());
                }
                Task::none()
            }
            Message::OpenConfig => {
                if self.current_window == Screens::EditorScreen {
                    self.current_window = Screens::ConfigScreen;
                } else {
                    self.current_window = Screens::EditorScreen;
                }
                Task::none()
            },
            Message::PaneResized(pane_grid::ResizeEvent {ratio, split}) => {
                self.panes.resize(split, ratio);                
                Task::none()
            },
            Message::Events(event) => {
                match event {
                    iced::Event::Keyboard(key) => {
                        match key {
                            iced::keyboard::Event::KeyPressed {key, modified_key, physical_key, location, modifiers, text, repeat} => {
                                if modifiers == Modifiers::CTRL && key == Key::Character("b".into()) {
                                    self.tree_view_is_closed = !self.tree_view_is_closed;
                                    if !self.tree_view_is_closed {
                                        self.panes.restore();
                                    } else {
                                        self.panes.maximize(self.editor_pane);
                                    }
                                }
                                
                                if modifiers == Modifiers::CTRL && key == Key::Character("s".into()) {
                                    save_file(&self.current_file_path, self.content.text());
                                }
                                
                            },
                            _=> {
                                
                            }
                        }
                    },
                    _=> {
                        
                    }
                }
                Task::none()
            },
            _=> {
                Task::none()
            }
        }
    }
}