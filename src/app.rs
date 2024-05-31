use std::error;

use getset::{Getters, Setters};
use ratatui::{layout::Rect, Frame};

use crate::{cli::Cli, csv_data::CsvData, log_view::LogView, LogData, LogViewState, ViewPort};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct App {
    /// Is the application running?
    running: bool,

    cli: Cli,

    data: CsvData,

    viewstate: LogViewState,

    page_size: u16,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(cli: Cli) -> anyhow::Result<Self> {
        let data = CsvData::try_from(cli.file().path())?;
        let viewstate = LogViewState::default();
        Ok(Self {
            running: true,
            cli,
            data,
            viewstate,
            page_size: 1,
        })
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn forward(&mut self, steps: usize) {
        if !self.data().is_empty() {
            self.viewstate.set_vscroll_offset(usize::min(
                *self.viewstate().vscroll_offset() + steps,
                self.data().len() - 1,
            ));
        }
    }

    pub fn backward(&mut self, steps: usize) {
        self.viewstate
            .set_vscroll_offset(usize::max(*self.viewstate.vscroll_offset(), steps) - steps);
    }

    pub fn begin(&mut self) {
        self.viewstate.set_vscroll_offset(0);
    }

    pub fn end(&mut self) {
        if !self.data().is_empty() {
            self.viewstate.set_vscroll_offset(self.data().len() - 1);
        }
    }

    pub fn right(&mut self, steps: usize) {
        let viewport = ViewPort::new(self.viewstate.hscroll_offset() + steps, 0, 44, 55);
        let width: usize = self.data.data_widths(&viewport).sum();
        if width > 0 {
            self.viewstate
                .set_hscroll_offset(self.viewstate.hscroll_offset() + steps);
        }
    }

    pub fn left(&mut self, steps: usize) {
        if *self.viewstate.hscroll_offset() >= steps {
            self.viewstate
                .set_hscroll_offset(self.viewstate.hscroll_offset() - steps);
        } else {
            self.viewstate.set_hscroll_offset(0);
        }
    }

    pub fn render_log_contents(&mut self, frame: &mut Frame, area: Rect) {
        let mut viewstate = *self.csv_viewstate();
        frame.render_stateful_widget(self.csv_contents(), area, &mut viewstate);
        self.viewstate = viewstate;
    }

    pub fn csv_contents(&self) -> LogView<CsvData> {
        LogView::from(&self.data)
    }

    pub fn csv_viewstate(&self) -> &LogViewState {
        &self.viewstate
    }
}
