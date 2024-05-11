use std::error;

use getset::Getters;

use crate::{cli::Cli, csv_view::{CsvView, CsvViewState}, reader::CsvData};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct App {
    /// Is the application running?
    running: bool,

    cli: Cli,

    data: CsvData,

    viewstate: CsvViewState
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(cli: Cli) -> anyhow::Result<Self> {
        let data = CsvData::try_from(cli.csv_file().path())?;
        let viewstate = CsvViewState::default();
        Ok(Self {
            running: true,
            cli,
            data,
            viewstate
        })
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn forward(&mut self, steps: usize) {
        self.viewstate.set_offset(usize::min(*self.viewstate().offset() + steps, self.data().len()));
    }

    pub fn backward(&mut self, steps: usize){
        self.viewstate.set_offset(usize::max(*self.viewstate.offset(), steps) - steps);
    }

    pub fn csv_contents(&self) -> CsvView {
        CsvView::from(&self.data)
    }

    pub fn csv_viewstate(&self) -> &CsvViewState {
        &self.viewstate
    }
}
