/// Display interface
pub mod tui;

use crate::types::{self, Programable};
use std::error::Error;

/// UI options
pub enum UI {
    TUI,
    GUI,
}

/// Run the `app` of type `Runnable`.
pub fn run_app(app: &dyn types::Runnable) -> Result<(), Box<dyn Error>> {
    app.run()
}

/// Run TUI or GUI mode depending on `ui_type`.
/// GUI is not implemented as of now.
pub fn run_ui<T: Programable>(ui_type: UI, items: Vec<T>) {
    let result = match ui_type {
        UI::TUI => {
            let app = tui::App::new(items.clone());
            run_app(&app)
        }
        UI::GUI => {
            panic!("Not implemented yet")
        }
    };

    if let Err(error) = result {
        panic!("{}", error);
    }
}
