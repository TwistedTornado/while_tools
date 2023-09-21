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
}

#[derive(Copy, Clone, Debug)]
pub struct FilePos2d {
    row: usize,
    col: usize,
}
