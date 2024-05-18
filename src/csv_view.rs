use getset::{Getters, Setters};
use ratatui::widgets::{Block, BorderType, Borders, StatefulWidget, Table};

use crate::csv_data::CsvData;

pub struct CsvView<'d> {
    data: &'d CsvData,
}

#[derive(Debug, Default, Clone, Copy, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct CsvViewState {
    vscroll_offset: usize,
    hscroll_offset: usize,
}

impl<'d> From<&'d CsvData> for CsvView<'d> {
    fn from(data: &'d CsvData) -> Self {
        Self { data }
    }
}

impl<'d> StatefulWidget for CsvView<'d> {
    type State = CsvViewState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let table = Table::new(
            self.data
                .row_window(state.vscroll_offset, area.height as usize),
            self.data.column_lengths().map(|s| *s as u16),
        )
        .block(
            Block::new()
                .borders(Borders::RIGHT)
                .border_type(BorderType::Rounded),
        );

        ratatui::widgets::Widget::render(table, area, buf);
    }
}
