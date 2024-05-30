use getset::Getters;
use ratatui::widgets::Row;


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

    fn columns(&self) -> usize;
    fn column_info(&self, idx: usize) -> Option<&ColumnInfo>;
    fn iter_columns(&self) -> impl Iterator<Item = &ColumnInfo>;
    fn rows(&self, first: usize, count: usize) -> impl Iterator<Item = Row<'_>>;
}
