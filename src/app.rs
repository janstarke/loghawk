use std::error;

use getset::{Getters, Setters};

use crate::{cli::Cli, csv_view::{CsvView, CsvViewState}, csv_data::CsvData};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug, Getters, Setters)]
#[getset(get = "pub", set="pub")]
pub struct App {
    /// Is the application running?
    running: bool,

    cli: Cli,

    data: CsvData,

    viewstate: CsvViewState,

    page_size: u16
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
            viewstate,
            page_size: 1
        })
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn forward(&mut self, steps: usize) {
        if ! self.data().is_empty() {
            self.viewstate.set_offset(usize::min(*self.viewstate().offset() + steps, self.data().len()-1));
        }
    }

    pub fn backward(&mut self, steps: usize){
        self.viewstate.set_offset(usize::max(*self.viewstate.offset(), steps) - steps);
    }

    pub fn begin(&mut self) {
        self.viewstate.set_offset(0);
    }

    pub fn end(&mut self) {
        if ! self.data().is_empty() {
            self.viewstate.set_offset(self.data().len()-1);
        }
    }

    pub fn csv_contents(&self) -> CsvView {
        CsvView::from(&self.data)
    }

    pub fn csv_viewstate(&self) -> &CsvViewState {
        &self.viewstate
    }
}
