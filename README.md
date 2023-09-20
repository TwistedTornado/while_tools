# While Tools

## Description

A set of tools for While, a minimal language used at university for teaching
foundational concepts about computation and correctness. It is heavily inspired
by a language described by C. A. Hoare in
[An Axiomatic Basis for Computer Programming](https://dl.acm.org/doi/10.1145/363235.363259)
.

Right now, the tools consist only of:

- `while_tools::lexer` -- a module for lexing
- `while_tools::ast` -- a module for working with an AST representation of a
  While program
- `while_tools::parser` -- a module for parsing a token stream into an AST
- `while_tools::interpreter` -- a module for interpreting ASTs of While
  programs.
