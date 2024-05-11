use getset::{Getters, Setters};
use ratatui::{
    layout::{Constraint, Layout},
    widgets::{List, StatefulWidget},
};

use crate::reader::CsvData;

pub struct CsvView<'d> {
    data: &'d CsvData,
}

#[derive(Debug, Default, Clone, Copy, Getters, Setters)]
#[getset(get="pub", set="pub")]
pub struct CsvViewState {
    offset: usize,
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
            .window(state.offset, area.height as usize)
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
        let parts = Layout::horizontal(vec![Constraint::Min(1), Constraint::Min(1)]).split(area);

        let first_column = List::new(first);
        ratatui::widgets::Widget::render(first_column, parts[0], buf);

        let remaining_column = List::new(remaining.into_iter().map(|r| r.join("|")));
        ratatui::widgets::Widget::render(remaining_column, parts[1], buf)
    }
}
