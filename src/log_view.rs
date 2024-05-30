use getset::{Getters, Setters};
use ratatui::{
    layout::Constraint, style::{Style, Stylize}, text::Text, widgets::{Block, BorderType, Borders, StatefulWidget, Table}
};

use crate::LogData;

pub struct LogView<'d, D>
where
    D: LogData,
{
    data: &'d D,
}

#[derive(Debug, Default, Clone, Copy, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct CsvViewState {
    vscroll_offset: usize,
    hscroll_offset: usize,
}

impl<'d, D> From<&'d D> for LogView<'d, D>
where
    D: LogData,
{
    fn from(data: &'d D) -> Self {
        Self { data }
    }
}

impl<'d, D> StatefulWidget for LogView<'d, D>
where
    D: LogData,
{
    type State = CsvViewState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let widths: Vec<_> = self
            .data
            .iter_columns()
            .map(|c| u16::try_from(*c.width()).unwrap())
            .collect();

        let table = Table::new(
            self.data.rows(state.vscroll_offset, area.height as usize),
            self.data
                .iter_columns()
                .map(|c| Constraint::Min(u16::try_from(*c.width()).unwrap())),
        )
        .block(
            Block::new()
                .borders(Borders::RIGHT)
                .border_type(BorderType::Rounded),
        );

        ratatui::widgets::Widget::render(table, area, buf);
        ratatui::widgets::Widget::render(Text::raw(format!("{widths:?}")), area, buf);
    }
}
