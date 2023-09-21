use crate::lexer::Span;
use std::iter;

/// Used by error-handling code to find cartesian positions within the source,
/// such as row and column. This is not needed to find the source tokens corresponding
/// to a span themselves, but rather their 2D position within a source file
pub struct SourceNavigator<'a> {
    source: &'a str,
    line_heads: Vec<usize>,
}

impl<'a> SourceNavigator<'a> {
    pub fn new(source: &'a str) -> Self {
        // Line heads are the start indices of each line.
        let line_heads: Vec<_> = iter::once(0)
            .chain(
                source
                    .chars()
                    .into_iter()
                    .enumerate()
                    .filter(|(_, c)| c == &'\n')
                    .map(|(idx, _)| idx + 1),
            )
            .collect();

        Self { source, line_heads }
    }

    /// Gives the line and column of a particular index.
    pub fn get_position(&self, index: usize) -> FilePos2d {
        let (this_line_index, (this_start, _)) = self
            .line_heads
            .iter()
            .zip(self.line_heads.iter().skip(1))
            .enumerate()
            .find(|(_, (_, next_head))| !(**next_head < index))
            .unwrap();

        FilePos2d {
            row: this_line_index,
            col: index - this_start,
        }
    }

    /// Given a line number, returns that line in the source stream.
    /// Trims newlines.
    pub fn get_line(&self, line_index: usize) -> &'a str {
        let this_line_start = self.line_heads[line_index];
        let next_line_start = self
            .line_heads
            .get(line_index + 1)
            .unwrap_or(&self.source.len())
            .to_owned();

        &self.source[this_line_start..next_line_start - 1].trim()
    }

    /// Given a span, returns the line containing that span, with the span
    /// content underlined.
    pub fn get_annotated_span(&self, Span(a, b): Span) -> String {
        let FilePos2d {
            row: row1,
            col: col1,
        } = self.get_position(a);

        let FilePos2d {
            row: _row2,
            col: col2,
        } = self.get_position(b);

        // The big assumption in this function is that the span doesn't cross
        // lines.

        let line_content = self.get_line(row1);

        let line_start = self.line_heads[row1];
        let line_end = line_start + line_content.len();

        let start_to_span = a - line_start;
        let span_to_end = if line_end < b { 0 } else { line_end - b };

        let line_marker = format!("{} | ", row1 + 1);

        let underline = " ".repeat(start_to_span + line_marker.len())
            + &"-".repeat(col2 - col1)
            + &" ".repeat(span_to_end);

        format!("{line_marker}{line_content}\n{underline}")
    }
}

#[derive(Copy, Clone, Debug)]
pub struct FilePos2d {
    row: usize,
    col: usize,
}
