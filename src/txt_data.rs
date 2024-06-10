use std::io::{BufRead, BufReader};

use clio::{ClioPath, Input};
use ratatui::{
    text::Line,
    widgets::{Cell, ListItem, Row},
};

use crate::{
    log_line::LogLine, AsMasked, ColumnInfo, ColumnWidth, DataRows, IndexRows, IterDataColumns,
    LogData,
};

#[derive(Debug)]
pub struct TxtData {
    lines: Vec<LogLine>,

    index_info: ColumnInfo,
    contents_info: ColumnInfo,
}

impl TxtData {
    pub fn load_from(path: &ClioPath, delimiter: char) -> anyhow::Result<Self> {
        let input = Input::new(path.as_os_str())?;
        let reader = BufReader::new(input);
        let mut lines = Vec::new();
        let mut index_width = 0;
        let mut contents_width = 0;
        for line in reader.lines() {
            let line = line?;

            if let Some((index, content)) = line.split_once(delimiter) {
                let index = index.to_string();
                let content = content.to_string();

                index_width = usize::max(index.len(), index_width);
                contents_width = usize::max(content.len(), contents_width);

                lines.push(LogLine::new(index, content)?);
            } else {
                lines.push(LogLine::new("".into(), line)?);
            }
        }

        Ok(Self {
            lines,
            index_info: ColumnInfo::new(ColumnWidth::Width(index_width)),
            contents_info: ColumnInfo::new(ColumnWidth::Width(contents_width)),
        })
    }
}

impl LogData for TxtData {
    fn len(&self) -> usize {
        self.lines.len()
    }

    fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    fn index_info(&self) -> &crate::ColumnInfo {
        &self.index_info
    }

    fn data_columns(&self) -> usize {
        1
    }

    fn data_infos(&self, idx: usize) -> Option<&crate::ColumnInfo> {
        if idx == 0 {
            Some(&self.contents_info)
        } else {
            None
        }
    }

    fn iter_data_columns(&self) -> crate::IterDataColumns<'_> {
        IterDataColumns::from(vec![&self.contents_info].into_iter())
    }

    fn index_rows(&self, viewport: &crate::ViewPort, mask_unicode: bool) -> crate::IndexRows<'_> {
        let upper_bound = usize::min(self.lines.len(), viewport.vend());
        IndexRows::from(
            self.lines[viewport.vbegin()..upper_bound]
                .iter()
                .map(move |v| ListItem::new(v.key_value().as_masked(.., mask_unicode))),
        )
    }

    fn data_rows(&self, viewport: &crate::ViewPort, mask_unicode: bool) -> crate::DataRows<'_> {
        let upper_bound = usize::min(self.lines.len(), viewport.vend());
        let hoffset = *viewport.hoffset();
        DataRows::from(
            self.lines[viewport.vbegin()..upper_bound]
                .iter()
                .map(move |v| {
                    Row::new(vec![Cell::new(match v.contents(0) {
                        None => Line::raw(""),
                        Some(line) => {
                            if hoffset >= line.len() {
                                Line::raw("")
                            } else {
                                line.as_masked(hoffset.., mask_unicode)
                            }
                        }
                    })])
                }),
        )
    }
}
