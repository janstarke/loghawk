use anyhow::bail;
use clio::ClioPath;
use ratatui::{
    text::Line,
    widgets::{Cell, ListItem, Row},
};
use std::fmt::Debug;

use crate::{
    log_line::LogLine, AsMasked, ColumnInfo, ColumnWidth, DataRows,
    DataWidths, IndexRows, InputReader, IterDataColumns, LogData, ViewPort,
};

pub struct CsvData {
    records: Vec<LogLine>,
    columns: Vec<ColumnInfo>,
}

impl CsvData {
    fn find_start(&self, viewport: &ViewPort) -> (usize, usize) {
        assert!(self.columns.len() >= 2);

        let mut skip = *viewport.hoffset();
        for (idx, width) in self
            .iter_data_columns()
            .map(|c| usize::try_from(*c.width()).unwrap())
            .enumerate()
        {
            if skip < width {
                return (idx, skip);
            } else {
                skip -= width;
            }
        }
        (self.columns.len() - 1, skip)
    }
}

impl LogData for CsvData {
    fn data_columns(&self) -> usize {
        self.columns.len()
    }

    fn data_infos(&self, idx: usize) -> Option<&ColumnInfo> {
        self.columns.get(idx + 1)
    }

    fn data_widths(&self, viewport: &ViewPort) -> DataWidths<'_> {
        let (first_column_index, skip_in_column) = self.find_start(viewport);
        DataWidths::from(
            self.iter_data_columns()
                .skip(first_column_index)
                .map(|c| usize::try_from(*c.width()).unwrap())
                .enumerate()
                .map(move |(idx, width)| {
                    if idx == 0 {
                        if width > skip_in_column {
                            width - skip_in_column
                        } else {
                            0
                        }
                    } else {
                        width
                    }
                }),
        )
    }

    fn data_rows(&self, viewport: &ViewPort, mask_unicode: bool) -> DataRows<'_> {
        let (first_column_index, skip_in_column) = self.find_start(viewport);

        let upper_bound = usize::min(self.records.len(), viewport.vend());
        DataRows::from(
            self.records[viewport.vbegin()..upper_bound]
                .iter()
                .map(move |r| {

                    let row = Row::new(r.iter_contents().skip(first_column_index).enumerate().map(
                        |(idx, value)| {
                            Cell::new(if idx == 0 {
                                if skip_in_column >= value.len() {
                                    Line::raw("")
                                } else {
                                    value.as_masked(skip_in_column.., mask_unicode)
                                }
                            } else {
                                value.as_masked(.., mask_unicode)
                            })
                        },
                    ));

                    row
                }),
        )
    }

    fn index_rows(&self, viewport: &ViewPort, mask_unicode: bool) -> IndexRows<'_> {
        let upper_bound = usize::min(self.records.len(), viewport.vend());
        IndexRows::from(
            self.records[viewport.vbegin()..upper_bound]
                .iter()
                .map(move |r| ListItem::new(r.key_value().as_masked(.., mask_unicode))),
        )
    }

    fn len(&self) -> usize {
        self.records.len()
    }

    fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    fn iter_data_columns(&self) -> IterDataColumns<'_> {
        IterDataColumns::from(self.columns.iter().skip(1))
    }

    fn index_info(&self) -> &ColumnInfo {
        self.columns.first().unwrap()
    }
}

impl TryFrom<&ClioPath> for CsvData {
    type Error = anyhow::Error;

    fn try_from(path: &ClioPath) -> Result<Self, Self::Error> {
        let mut reader = csv::Reader::from_reader(InputReader::try_from(path)?);
        let mut records = Vec::new();
        let mut columns = Vec::new();

        for record in reader.records() {
            let record = record?;

            if columns.is_empty() {
                columns = vec![ColumnWidth::Width(0); record.iter().count()];
            } else {
                for (idx, s) in record.iter().enumerate() {
                    columns[idx].advance_to(s.len());
                }
            }
            let line = LogLine::try_from(record)?;
            records.push(line);
        }

        let columns: Vec<_> = columns.into_iter().map(ColumnInfo::new).collect();
        if columns.len() < 2 {
            if columns.len() == 1 {
                bail!("found a key column, but no data columns");
            } else {
                bail!("found no data at all");
            }
        }

        Ok(Self {
            records,
            columns,
        })
    }
}

impl Debug for CsvData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CsvData")
            .field("records", &self.records.len())
            .finish_non_exhaustive()
    }
}
