use getset::Getters;
use ratatui::widgets::{ListItem, Row};

use crate::ViewPort;

#[derive(Clone, Copy, Debug)]
pub enum ColumnWidth {
    Width(usize),
    Unknown,
}

impl ColumnWidth {
    pub fn advance_to(&mut self, width: usize) {
        if let Self::Width(me) = self {
            *me = usize::max(*me, width);
        }
    }
}

impl TryFrom<ColumnWidth> for usize {
    type Error = anyhow::Error;

    fn try_from(width: ColumnWidth) -> Result<Self, Self::Error> {
        match width {
            ColumnWidth::Width(width) => Ok(width),
            ColumnWidth::Unknown => anyhow::bail!("this column has no width"),
        }
    }
}

impl TryFrom<ColumnWidth> for u16 {
    type Error = anyhow::Error;

    fn try_from(width: ColumnWidth) -> Result<Self, Self::Error> {
        match width {
            ColumnWidth::Width(width) => Ok(u16::try_from(width)?),
            ColumnWidth::Unknown => anyhow::bail!("this column has no width"),
        }
    }
}

#[derive(Getters)]
#[getset(get = "pub")]
pub struct ColumnInfo {
    width: ColumnWidth,
}

impl ColumnInfo {
    pub fn new(width: ColumnWidth) -> Self {
        Self { width }
    }
}

pub trait LogData {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;

    fn index_info(&self) -> &ColumnInfo;
    fn data_columns(&self) -> usize;
    fn data_infos(&self, idx: usize) -> Option<&ColumnInfo>;
    fn iter_data_columns(&self) -> impl Iterator<Item = &ColumnInfo>;
    fn index_rows(&self, viewport: &ViewPort) -> impl Iterator<Item = ListItem<'_>>;
    fn data_rows(&self, viewport: &ViewPort) -> impl Iterator<Item = Row<'_>>;

    fn data_widths(&self, _viewport: &ViewPort) -> impl Iterator<Item = usize> {
        self.iter_data_columns()
            .map(|c| usize::try_from(*c.width()).unwrap())
    }
}
