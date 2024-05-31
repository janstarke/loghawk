/// Application.
pub mod app;

/// Terminal events handler.
pub mod event;

/// Widget renderer.
pub mod ui;

/// Terminal user interface.
pub mod tui;

/// Event handler.
pub mod handler;

pub mod cli;
mod csv_data;
mod txt_data;
mod input_reader;
mod log_view;
mod log_view_state;
mod log_data;
mod viewport;

pub use csv_data::*;
pub use txt_data::*;
pub use input_reader::*;
pub use log_data::*;
pub use log_view_state::*;
pub use viewport::*;

pub mod tui_helper;