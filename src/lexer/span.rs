//! Spans for locating tokens within the source.
//!
//! Use `Spanned<T>` to associate a `T` with its span, and use `Span` for code
//! that doesn't need the `T`.

#[derive(Copy, Clone, Debug)]
pub struct Spanned<T> {
    pub inner: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(inner: T, span: Span) -> Self {
        Self { inner, span }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Span(pub usize, pub usize);
