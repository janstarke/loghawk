use std::ops::RangeBounds;

use ratatui::{
    style::{Color, Stylize},
    text::{Line, Span},
};
use unicode_segmentation::{GraphemeIndices, UnicodeSegmentation};
use unicode_width::UnicodeWidthStr;

const INTERESTING_STRINGS: [&str; 2] = ["krbtgt", "admin"];

pub trait AsMasked {
    fn as_masked<R: RangeBounds<usize> + std::slice::SliceIndex<str>>(
        &self,
        range: R,
        mask_unicode: bool,
    ) -> Line<'_>;
}

impl AsMasked for String {
    fn as_masked<R: RangeBounds<usize> + std::slice::SliceIndex<str>>(
        &self,
        range: R,
        mask_unicode: bool,
    ) -> Line<'_> {
        do_as_masked(self, range, &INTERESTING_STRINGS, mask_unicode)
    }
}

impl AsMasked for str {
    fn as_masked<R: RangeBounds<usize> + std::slice::SliceIndex<str>>(
        &self,
        range: R,
        mask_unicode: bool,
    ) -> Line<'_> {
        do_as_masked(self, range, &INTERESTING_STRINGS, mask_unicode)
    }
}

fn do_as_masked<'a, R: RangeBounds<usize> + std::slice::SliceIndex<str>>(
    s: &'a str,
    range: R,
    interesting_strings: &[&str],
    mask_unicode: bool,
) -> Line<'a> {
    let l = s.to_lowercase();

    let mut is_interesting = false;
    for i in interesting_strings {
        if l.contains(i) {
            is_interesting = true;
            break;
        }
    }

    let line = if mask_unicode {
        Line::from(s.iter_spans(range).collect::<Vec<Span>>())
    } else {
        Line::from(
            s.grapheme_indices(true)
                .filter_map(|(idx, s)| if range.contains(&idx) { Some(s) } else { None })
                .collect::<String>(),
        )
    };

    if is_interesting {
        line.red().on_black()
    } else {
        line
    }
}

trait IterSpans<R: RangeBounds<usize> + std::slice::SliceIndex<str>> {
    fn iter_spans(&self, range: R) -> MaskedSpans<'_, R>;
}

impl<R> IterSpans<R> for str
where
    R: RangeBounds<usize> + std::slice::SliceIndex<str>,
{
    fn iter_spans(&self, range: R) -> MaskedSpans<'_, R> {
        MaskedSpans {
            graphemes: self.grapheme_indices(true),
            range,
            defered_span: None,
        }
    }
}

struct MaskedSpans<'a, R>
where
    R: RangeBounds<usize> + std::slice::SliceIndex<str>,
{
    graphemes: GraphemeIndices<'a>,
    range: R,
    defered_span: Option<Span<'a>>,
}
impl<'a, R> Iterator for MaskedSpans<'a, R>
where
    R: RangeBounds<usize> + std::slice::SliceIndex<str>,
{
    type Item = Span<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current_span_content = Vec::new();

        if let Some(defered_span) = self.defered_span.take() {
            return Some(defered_span);
        }

        for (idx, grapheme_cluster) in self.graphemes.by_ref() {
            if self.range.contains(&idx) {
                if grapheme_cluster.len() == 1 {
                    let ch = grapheme_cluster.chars().next().unwrap();

                    if ch.is_alphanumeric() || ch.is_whitespace() || ch.is_ascii() {
                        current_span_content.push(ch)
                    } else {
                        let highlighted_span = Span::raw(ch.escape_unicode().collect::<String>())
                            .fg(Color::LightYellow)
                            .bg(Color::Red);

                        if current_span_content.is_empty() {
                            return Some(highlighted_span);
                        } else {
                            self.defered_span = Some(highlighted_span);
                            break;
                        }
                    }
                } else if grapheme_cluster.width() < 1 {
                    let highlighted_span = Span::raw(
                        grapheme_cluster
                            .chars()
                            .map(|ch| ch.escape_unicode().to_string())
                            .collect::<String>(),
                    )
                    .fg(Color::LightYellow)
                    .bg(Color::Red);

                    if current_span_content.is_empty() {
                        return Some(highlighted_span);
                    } else {
                        self.defered_span = Some(highlighted_span);
                        break;
                    }
                } else {
                    // might be something like 'y̆', which is a combination of 'y' and '\u{0306}'
                    current_span_content.extend(grapheme_cluster.chars())
                }
            }
        }

        if current_span_content.is_empty() {
            None
        } else {
            Some(Span::raw(
                current_span_content.into_iter().collect::<String>(),
            ))
        }
    }
}
