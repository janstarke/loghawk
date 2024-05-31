use ratatui::{
    widgets::{Block, BorderType, Borders},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let block = Block::new()
        .borders(Borders::NONE)
        .border_type(BorderType::Rounded);
    app.set_page_size(block.inner(frame.size()).height);
    app.render_log_contents(frame, block.inner(frame.size()));
    frame.render_widget(block, frame.size());
}
