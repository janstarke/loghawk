use getset::{Getters, Setters};
use ratatui::layout::Rect;

use crate::ViewPort;


#[derive(Debug, Default, Clone, Copy, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct LogViewState {
    vscroll_offset: usize,
    hscroll_offset: usize,
}

impl LogViewState {
    pub fn viewport(&self, area: &Rect) -> ViewPort {
        ViewPort::from_rect(self.hscroll_offset, self.vscroll_offset, area)
    }
}