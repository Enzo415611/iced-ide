use std::{fmt::Debug, path::PathBuf};

use iced::{Size, Theme, widget::{pane_grid, text_editor}};


mod ui;
mod file;
mod update;


fn main() -> iced::Result {
     iced::application(App::boot, App::update, App::view)
        .title("IDE")
        .settings(iced::Settings {
            id: Some("IDE".to_string()),
            ..Default::default() 
        })
        .resizable(true)
        .window_size(Size::new(800., 600.))
        .theme(App::theme)
        .run()
}

#[derive(PartialEq)]
enum Screens {
    EditorScreen,
    ConfigScreen,
}

enum Pane {
    EditorPane,
    TreeViewPane,
}

#[derive(Debug, Clone)]
enum Message {
    ButtonTest,
    OpenFile,
    Edit(text_editor::Action),
    SaveFile,
    OpenConfig,
    PaneResized(pane_grid::ResizeEvent),
}


struct App {
    current_window: Screens,
    content: text_editor::Content,
    current_file_path: PathBuf,
    current_file_extension: String,
    app_theme: iced::Theme,
    panes: pane_grid::State<Pane>
}

impl App {
    fn boot() -> Self {
        let (mut panes, pane) = pane_grid::State::new(Pane::TreeViewPane);
        
        _=panes.split(pane_grid::Axis::Vertical, pane, Pane::EditorPane);
        let text_editor_content = text_editor::Content::new();
        Self {
            content: text_editor_content,
            current_window: Screens::EditorScreen,
            current_file_path: PathBuf::new(),
            current_file_extension: String::new(),
            app_theme: Theme::Dark,
            panes
        }
    }
    
    fn theme(&self) -> iced::Theme {
        self.app_theme.clone()
    }
}