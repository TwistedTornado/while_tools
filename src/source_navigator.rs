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
}
