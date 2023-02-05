/// Display interface
pub mod tui;

use crate::types::{self, Programable};
use std::error::Error;

pub enum UI {
    TUI,
    GUI
}

pub fn run_app(app: &dyn types::Runnable) -> Result<(), Box<dyn Error>> {
    app.run()
}

pub fn run_ui<T: Programable>(ui_type: UI, items: Vec<T>) {
    let result = match ui_type {
        UI::TUI => {
            let app = tui::App::new(items.clone());
            run_app(&app)
        },
        UI::GUI => {
            panic!("Not implemented yet")
        }
    };

    if let Err(error) = result {
        panic!("{}", error);
    }
}
