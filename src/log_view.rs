use ratatui::{
    layout::{Constraint, Layout, Margin},   
    widgets::{Block, BorderType, Borders, List, StatefulWidget, Table},
};

use crate::{LogData, LogViewState};

pub struct LogView<'d, D>
where
    D: LogData,
{
    data: &'d D,
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
    type State = LogViewState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let margin = Margin::new(0, 0);
        let widths: Vec<_> = self
            .data
            .iter_columns()
            .map(|c| u16::try_from(*c.width()).unwrap())
            .collect();

        let index_width = widths.first().unwrap_or(&0) + 2*margin.horizontal + 1; // add 1 to have space for the right border

        let parts = Layout::horizontal(vec![Constraint::Length(index_width), Constraint::Min(1)])
            .split(area);
        let index_part = parts[0].inner(&margin);
        let data_part = parts[1].inner(&margin);

        let index_list = List::new(
            self.data
                .index_rows(&state.viewport(&index_part)),
        )
        .block(
            Block::new()
                .borders(Borders::RIGHT)
                .border_type(BorderType::Rounded),
        );

        let data_table = Table::new(
            self.data.rows(&state.viewport(&data_part)),
            self.data
                .iter_columns()
                .map(|c| Constraint::Min(u16::try_from(*c.width()).unwrap())),
        )
        .block(
            Block::new()
                .borders(Borders::NONE)
                .border_type(BorderType::Rounded),
        );

        ratatui::widgets::Widget::render(index_list, index_part, buf);
        ratatui::widgets::Widget::render(data_table, data_part, buf);
    }
}
