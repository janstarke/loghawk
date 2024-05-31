use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clio::ClioPath;
use ratatui::widgets::{Cell, ListItem, Row};

use crate::{ColumnInfo, ColumnWidth, DataRows, IndexRows, IterDataColumns, LogData};

#[derive(Debug)]
pub struct TxtData {
    indices: Vec<String>,
    contents: Vec<String>,

    index_info: ColumnInfo,
    contents_info: ColumnInfo,
}

impl TxtData {
    pub fn load_from(path: &ClioPath, delimiter: char) -> anyhow::Result<Self> {
        let reader = BufReader::new(File::open(path.path())?);
        let mut indices = Vec::new();
        let mut contents = Vec::new();
        let mut index_width = 0;
        let mut contents_width = 0;
        for line in reader.lines() {
            let line = line?;
            if let Some((index, content)) = line.split_once(delimiter) {
                let index = index.to_string();
                let content = content.to_string();

                index_width = usize::max(index.len(), index_width);
                contents_width = usize::max(content.len(), contents_width);

                indices.push(index);
                contents.push(content);
            } else {
                indices.push("".into());
                contents.push(line);
            }
        }

        assert_eq!(indices.len(), contents.len());

        Ok(Self {
            indices,
            contents,
            index_info: ColumnInfo::new(ColumnWidth::Width(index_width)),
            contents_info: ColumnInfo::new(ColumnWidth::Width(contents_width)),
        })
    }
}

impl LogData for TxtData {
    fn len(&self) -> usize {
        self.indices.len()
    }

    fn is_empty(&self) -> bool {
        self.indices.is_empty()
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

    fn index_rows(&self, viewport: &crate::ViewPort) -> crate::IndexRows<'_> {
        let upper_bound = usize::min(self.indices.len(), viewport.vend());
        IndexRows::from(
            self.indices[viewport.vbegin()..upper_bound]
                .iter()
                .map(|v| ListItem::new(&v[..])),
        )
    }

    fn data_rows(&self, viewport: &crate::ViewPort) -> crate::DataRows<'_> {
        let upper_bound = usize::min(self.indices.len(), viewport.vend());
        let hoffset = *viewport.hoffset();
        DataRows::from(
            self.contents[viewport.vbegin()..upper_bound]
                .iter()
                .map(move |v| {
                    Row::new(
                    vec![Cell::new(if hoffset >= v.len() {
                        ""
                    } else {
                        &v[hoffset..]
                    })])
                }),
        )
    }
}
