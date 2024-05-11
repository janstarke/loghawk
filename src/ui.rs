use ratatui::{
    widgets::{Block, BorderType, Borders},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let contents = app.csv_contents();
    let mut viewstate = *app.csv_viewstate();

    frame.render_stateful_widget(contents, block.inner(frame.size()), &mut viewstate);
    frame.render_widget(block, frame.size());
}
