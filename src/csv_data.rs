use clio::ClioPath;
use csv::StringRecord;
use ratatui::widgets::{Cell, ListItem, Row};
use std::fmt::Debug;

use crate::{ColumnInfo, ColumnWidth, InputReader, LogData, ViewPort};

pub struct CsvData {
    records: Vec<StringRecord>,
    columns: Vec<ColumnInfo>,
}

impl LogData for CsvData {
    fn columns(&self) -> usize {
        self.columns.len()
    }

    fn column_info(&self, idx: usize) -> Option<&ColumnInfo> {
        self.columns.get(idx)
    }

    fn rows(&self, viewport: &ViewPort) -> impl Iterator<Item = Row<'_>> {
        let upper_bound = usize::min(self.records.len(), viewport.vend());
        self.records[viewport.vbegin()..upper_bound]
            .iter()
            .map(|r| Row::new(r.iter().map(Cell::new)))
    }

    fn index_rows(&self, viewport: &ViewPort) -> impl Iterator<Item = ListItem<'_>> {
        let upper_bound = usize::min(self.records.len(), viewport.vend());
        self.records[viewport.vbegin()..upper_bound]
            .iter()
            .map(|r| ListItem::new(r.get(0).unwrap_or_default()))
    }

    fn len(&self) -> usize {
        self.records.len()
    }
    fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    fn iter_columns(&self) -> impl Iterator<Item = &ColumnInfo> {
        self.columns.iter()
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
            records.push(record);
        }

        let columns = columns.into_iter().map(ColumnInfo::new).collect();

        Ok(Self { records, columns })
    }
}

impl Debug for CsvData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CsvData")
            .field("records", &self.records.len())
            .finish_non_exhaustive()
    }
}
