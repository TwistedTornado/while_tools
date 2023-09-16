//! Spans for locating tokens within the source.
//!
//! Use `Spanned<T>` to associate a `T` with its span, and use `Span` for code
//! that doesn't need the `T`.

#[derive(Copy, Clone, Debug)]
pub struct Spanned<T> {
    pub inner: T,
    pub span: Span,
}

#[derive(Copy, Clone, Debug)]
pub struct Span(pub usize, pub usize);
