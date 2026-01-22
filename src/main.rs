use std::{fmt::Debug, path::PathBuf};

use iced::{Event, Size, Subscription, Theme, event, widget::{image, pane_grid::{self}, text_editor}};


mod ui;
mod file;
mod update;


// image bytes 
const MENU_HAMBURGUER: &[u8] = include_bytes!("../src/assets/menu-hamburguer.png");
const OPEN_CONFIG: &[u8] = include_bytes!("../src/assets/controles-deslizantes-de-configuracoes.png");

fn main() {
    _=app();    
}

fn app() -> iced::Result {
    iced::application(App::boot, App::update, App::view)
       .title("IDE")
       .subscription(|app| app.subscription())
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

#[derive(Debug, Clone)]
pub enum Pane {
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
    Events(Event),
}


struct App {
    current_window: Screens,
    content: text_editor::Content,
    current_file_path: PathBuf,
    current_file_extension: String,
    app_theme: iced::Theme,
    panes: pane_grid::State<Pane>,
    editor_pane: pane_grid::Pane,
    tree_view_is_closed: bool,
    images: Vec<image::Handle>,
}



impl App {
    fn boot() -> Self {
        let (mut panes, tree_view_pane) = pane_grid::State::new(Pane::TreeViewPane);

        let split = panes.split(pane_grid::Axis::Vertical, tree_view_pane, Pane::EditorPane);
        let editor_pane = split.unwrap().0;
        
        // image handler
        let menu_hamburguer_image = image::Handle::from_bytes(MENU_HAMBURGUER);
        let open_menu_image = image::Handle::from_bytes(OPEN_CONFIG);
        
        let images = vec![menu_hamburguer_image, open_menu_image];
        
        Self {
            content: text_editor::Content::new(),
            current_window: Screens::EditorScreen,
            current_file_path: PathBuf::new(),
            current_file_extension: String::new(),
            app_theme: Theme::Dark,
            panes,
            editor_pane,
            tree_view_is_closed: false,
            images
        }
    }
    
    fn subscription(&self) -> Subscription<Message> {
        event::listen().map(|event| Message::Events(event))
    }
    
    fn theme(&self) -> iced::Theme {
        self.app_theme.clone()
    }    
}