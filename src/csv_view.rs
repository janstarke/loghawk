use getset::{Getters, Setters};
use ratatui::{
    layout::{Constraint, Layout},
    widgets::{Block, BorderType, Borders, List, StatefulWidget},
};

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
        let (first, remaining): (Vec<_>, Vec<_>) = self
            .data
            .window(state.vscroll_offset, area.height as usize)
            .filter_map(|record| {
                let mut iter = record.iter();
                match iter.next() {
                    Some(first) => {
                        let remaining: Vec<_> = iter.collect();
                        Some((first, remaining))
                    }
                    None => None,
                }
            })
            .unzip();
        let parts = Layout::horizontal(vec![
            Constraint::Length(*self.data.column_length(0).unwrap_or(&10) as u16),
            Constraint::Min(1),
        ])
        .split(area);

        let first_column = List::new(first).block(
            Block::new()
                .borders(Borders::RIGHT)
                .border_type(BorderType::Rounded),
        );
        ratatui::widgets::Widget::render(first_column, parts[0], buf);

        let remaining_contents = remaining.into_iter().map(|r| {
            let s = r.join(",").to_string();
            if state.hscroll_offset() >= &s.len() {
                String::default()
            } else {
                s[*state.hscroll_offset()..].to_string()
            }
        });
        let remaining_column = List::new(remaining_contents);
        let mut target_area = parts[1];
        ratatui::widgets::Widget::render(remaining_column, target_area, buf)
    }
}
