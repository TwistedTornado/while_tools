//! Spans for locating tokens within the source.
//!
//! Use `Spanned<T>` to associate a `T` with its span, and use `Span` for code
//! that doesn't need the `T`.

pub struct Spanned<T> {
    inner: T,
    span: Span,
}

pub struct Span(usize, usize);