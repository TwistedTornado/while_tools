//! # While Tools
//! A set of tools for While, a minimal language used at university for teaching
//! foundational concepts about computation and correctness. It is heavily
//! inspired by a language described by C. A. Hoare in
//! [An Axiomatic Basis for Computer Programming](https://dl.acm.org/doi/10.1145/363235.363259).
pub mod ast;
pub mod interpreter;
pub mod lexer;
pub mod parser;
mod source_navigator;

pub mod utils {
    use crate::source_navigator;
    pub use source_navigator::SourceNavigator;
}
