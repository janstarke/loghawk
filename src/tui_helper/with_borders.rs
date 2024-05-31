use ratatui::widgets::{Block, BorderType, Borders, List, Table, Widget};

pub trait WithBorders: Widget {
    fn with_borders(self, borders: Borders) -> Self;
}

impl WithBorders for Table<'_> {
    fn with_borders(self, borders: Borders) -> Self {
        self.block(
            Block::new().with_borders(borders)
        )
    }
}

impl WithBorders for List<'_> {
    fn with_borders(self, borders: Borders) -> Self {
        self.block(
            Block::new().with_borders(borders)
        )
    }
}

impl WithBorders for Block<'_> {
    fn with_borders(self, borders: Borders) -> Self {
        self.borders(borders).border_type(BorderType::Rounded)
    }
}
