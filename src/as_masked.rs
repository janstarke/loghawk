use std::ops::RangeBounds;

use ratatui::{
    style::{Color, Stylize},
    text::{Line, Span},
};

const INTERESTING_STRINGS: [&str; 2] = ["krbtgt", "admin"];

pub trait AsMasked {
    fn as_masked<R: RangeBounds<usize> + std::slice::SliceIndex<str>>(&self, range: R) -> Line<'_>;
}

impl AsMasked for String {
    fn as_masked<R: RangeBounds<usize> + std::slice::SliceIndex<str>>(&self, range: R) -> Line<'_> {
        do_as_masked(self, range, &INTERESTING_STRINGS)
    }
}

impl AsMasked for str {
    fn as_masked<R: RangeBounds<usize> + std::slice::SliceIndex<str>>(&self, range: R) -> Line<'_> {
        do_as_masked(self, range, &INTERESTING_STRINGS)
    }
}

fn do_as_masked<'a, R: RangeBounds<usize> + std::slice::SliceIndex<str>>(
    s: &'a str,
    range: R,
    interesting_strings: &[&str],
) -> Line<'a> {
    let mut spans = vec![];

    let l = s.to_lowercase();

    let mut is_interesting = false;
    for i in interesting_strings {
        if l.contains(i) {
            is_interesting = true;
            break;
        }
    }

    let mut current_span = vec![];
    for ch in s
        .chars()
        .enumerate()
        .filter_map(|(id, ch)| if range.contains(&id) { Some(ch) } else { None })
    {
        if ch.is_alphanumeric() || ch.is_whitespace() || ch.is_ascii() {
            current_span.push(ch)
        } else {
            let until_now: String = current_span.into_iter().collect();
            spans.push(Span::raw(until_now));

            let current_char: String = ch.escape_unicode().collect();
            spans.push(
                Span::raw(current_char)
                    .fg(Color::LightYellow)
                    .bg(Color::Red),
            );
            current_span = vec![];
        }
    }
    let until_now: String = current_span.into_iter().collect();
    spans.push(Span::raw(until_now));

    if is_interesting {
        Line::from(spans).red().on_black()
    } else {
        Line::from(spans)
    }
}

