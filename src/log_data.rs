use getset::Getters;
use ratatui::widgets::{ListItem, Row};
use std::fmt::Debug;

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

#[derive(Getters, Debug)]
#[getset(get = "pub")]
pub struct ColumnInfo {
    width: ColumnWidth,
}

impl ColumnInfo {
    pub fn new(width: ColumnWidth) -> Self {
        Self { width }
    }
}

macro_rules! wrap_iterator {
    ($clazz:ident, $item: ident) => {
        pub struct $clazz<'d>(Box<dyn Iterator<Item = $item> + 'd>);

        impl<'d> Iterator for $clazz<'d> {
            type Item = $item;

            fn next(&mut self) -> Option<Self::Item> {
                self.0.next()
            }
        }

        impl<'d> $clazz<'d>
        where
            Self: 'd,
        {
            pub fn from<I>(value: I) -> Self
            where
                I: Iterator<Item = $item> + 'd,
            {
                Self(Box::new(value))
            }
        }
    };
    ($clazz:ident, $item:ident < $r:lifetime >) => {
        pub struct $clazz<'d>(Box<dyn Iterator<Item = $item<'d>> + 'd>);

        impl<'d> Iterator for $clazz<'d> {
            type Item = $item<'d>;

            fn next(&mut self) -> Option<Self::Item> {
                self.0.next()
            }
        }

        impl<'d> $clazz<'d>
        where
            Self: 'd,
        {
            pub fn from<I>(value: I) -> Self
            where
                I: Iterator<Item = $item<'d>> + 'd,
            {
                Self(Box::new(value))
            }
        }
    };
}

pub struct IterDataColumns<'d>(Box<dyn Iterator<Item = &'d ColumnInfo> + 'd>);

impl<'d> IterDataColumns<'d>
where
    Self: 'd,
{
    pub fn from<I>(value: I) -> Self
    where
        I: Iterator<Item = &'d ColumnInfo> + 'd,
    {
        Self(Box::new(value))
    }
}

impl<'d> Iterator for IterDataColumns<'d> {
    type Item = &'d ColumnInfo;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

wrap_iterator!(IndexRows, ListItem<'_>);
wrap_iterator!(DataRows, Row<'_>);
wrap_iterator!(DataWidths, usize);

pub trait LogData: Debug {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;

    fn index_info(&self) -> &ColumnInfo;
    fn data_columns(&self) -> usize;
    fn data_infos(&self, idx: usize) -> Option<&ColumnInfo>;
    fn iter_data_columns(&self) -> IterDataColumns<'_>;
    fn index_rows(&self, viewport: &ViewPort) -> IndexRows<'_>;
    fn data_rows(&self, viewport: &ViewPort) -> DataRows<'_>;

    fn data_widths<'d>(&'d self, _viewport: &ViewPort) -> DataWidths<'d> {
        DataWidths::from(
            self.iter_data_columns()
                .map(|c| usize::try_from(*c.width()).unwrap()),
        )
    }
}
