use std::ops::RangeBounds;

use ratatui::{
    style::{Color, Stylize},
    text::{Line, Span},
};

pub trait AsMasked {
    fn as_masked<R: RangeBounds<usize> + std::slice::SliceIndex<str>>(&self, range: R) -> Line<'_>;
}

impl AsMasked for String {
    fn as_masked<R: RangeBounds<usize> + std::slice::SliceIndex<str>>(&self, range: R) -> Line<'_> {
        let mut spans = vec![];

        let mut current_span = vec![];
        for ch in
            self.chars()
                .enumerate()
                .filter_map(|(id, ch)| if range.contains(&id) { Some(ch) } else { None })
        {
            if ch.is_alphanumeric() || ch.is_whitespace() || ch.is_ascii() {
                current_span.push(ch)
            } else {
                let until_now: String = current_span.into_iter().collect();
                spans.push(Span::raw(until_now));

                let current_char: String = ch.escape_unicode().collect();
                spans.push(Span::raw(current_char).fg(Color::LightYellow).bg(Color::Red));
                current_span = vec![];
            }
        }
        let until_now: String = current_span.into_iter().collect();
        spans.push(Span::raw(until_now));
        Line::from(spans)
    }
}

impl AsMasked for str {
    fn as_masked<R: RangeBounds<usize> + std::slice::SliceIndex<str>>(&self, range: R) -> Line<'_> {
        let mut spans = vec![];

        let mut current_span = vec![];
        for ch in
            self.chars()
                .enumerate()
                .filter_map(|(id, ch)| if range.contains(&id) { Some(ch) } else { None })
        {
            if ch.is_alphanumeric() || ch.is_whitespace() || ch.is_ascii() {
                current_span.push(ch)
            } else {
                let until_now: String = current_span.into_iter().collect();
                spans.push(Span::raw(until_now));

                let current_char: String = ch.escape_unicode().collect();
                spans.push(Span::raw(current_char).fg(Color::LightYellow).bg(Color::Red));
                current_span = vec![];
            }
        }
        let until_now: String = current_span.into_iter().collect();
        spans.push(Span::raw(until_now));
        Line::from(spans)
    }
}
