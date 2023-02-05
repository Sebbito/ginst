/// Display interface
pub mod tui;

use crate::types;
use std::error::Error;

pub fn run_app(app: &dyn types::Runnable) -> Result<(), Box<dyn Error>> {
    app.run()
}
