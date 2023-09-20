This is not yet a complete spec, but it does list important design decisions and
things to consider.

The original language was small, with the below grammar:

```
S ::= x := a
    | skip
    | S1; S2
    | if b then S1 else S2
    | while b do S
    | (S)

b ::= true
    | false
    | a1 = a2
    | a1 <= a2
    | !b
    | b1 & b2
    | (b)

a ::= v
    | n
    | a1 + a2
    | a1 - a2
    | a1 * a2
    | (a)
```

## Design Decisions

In order to resolve ambiguity, a more complex grammar is used internally. While
I don't yet have a complete specification of the internal grammar, a few
important points are listed:

### Statement Block Greediness

`if b then T else F; S` will treat `F; S` all as part of the statement block
belonging to `else`. The same applies to `while b do W; S`. In order to
disambiguate this wrap the entire `if` or `while` statement in parentheses:

```
(if b then T else F); S
(while b do W); S
```

Support for this style being correct is the explicit instruction given to use
when disambiguating expressions in section 1.2 of the handbook:

```
x := 17; (while (0 <= x) do x := x − 1); x := x + 1
```

As well as example 1.2.5 in the handbook:

```
y := x − 1;
z := 1;

while (y > 1 ∧ z = 1 ) do (
    r := x;
    (while (y <= r) do r := r − y);
    (if (r = 0) then z := 0);
    y := y - 1;
)
```

Examples 1.2.4 and 1.2.2 break from this, however I have chosen to stick to this
style.

_Do note that even `while b do (W); S` is not enough; this will still
parse `(W); S` as one statement block._

### Implicit Semicolons

It's optional whether you end the line with a semicolon or not. The system for
automatic semicolon insertion I implemented is very basic. First, semicolons are
added in the lexing stage:

- When either an `\r` or an `\n` is encountered, the lexer eats as many such
  consecutive characters as it can as it can, and turns it into one `Semicolon`
  token.
- When a `;` character is encountered, that, along with all the consecutive `\r`
  and `\n` after it, are converted to one `Semicolon` token too.

### Everything is (nearly) an Expression

Internally, everything is part of the same `Ast` enum type. Though the language
grammar breaks the syntax down into 3 distinct categories (`Stmt`, `AExp`
and `BExp`), it makes certain things, like definitions, harder to handle. This
doesn't mean you can use statements as anything other than statements though, as
currently they all evaluate to a unit/empty type. Use of this type will cause
errors.

The parser doesn't yet properly treat statements as expressions, though I hope
to rewrite parts of it to respect this.

## Semantic Lowering

The parser aims to match the lowering of the transition system, rather than
using a different internal representation and simply producing correct outputs.

### Statements

- Statement blocks are not simply `Vec<Ast>`. Rather, every block of multiple
  statements is represented by a recursive nesting of `Comp` enum structs,
  analogous in structure to a cons list. This might change as more and more of
  the codebase uses iterators to interface with each other.
- The interpreter also supports the shorter `if` syntax provided by the
  handbook, which lowers to:

```
[[if b then T]] = [[if b then T else skip]]
```

- Lowering `while b do S` to `if b do (S; while b do S) else skip` is not as
  simple, because it can overflow the stack for even low-iteration loops. I have
  chosen not to implement this lowering for now.

### Comparisons

The handbook defines more comparisons in terms of `<=`, `!` and `=`. These are
semantically lowered in the same way:

- `[[a < b]] = [[!(b <= a)]]`
- `[[a >= b]] = [[b <= a]]`
- `[[a > b]] = [[!(a <= b)]]`
- `[[a != b]] = [[!(a = b)]]`

## Definitions

The interpreter supports definitions as they exist in the handbook. In there,
definitions are defined using the following syntax:

$$ W \triangleq \texttt{while $(y > 0)$ do $(x := x + x$; $y := y - 1)$} $$

This can be written in While as

```
W := [[while (y > 0) do (x := x + x; y := y - 1)]];
```

Subroutines can also be recursive:

```
W := [[x := x + 1; if x < 10 then W]];
W
```

Leads to `[x -> 10]`.

Like functions in Python, definitions can only be invoked if all the definitions
they referenced are already defined before invocation:

```
W := [[Y]]
W
Y := [[ ... ]]
```

Will not work, but

```
W := [[Y]]
Y := [[ ... ]]
W
```

will. This is due to my design decision to make the interpreter work on one pass
of the code.

## Caveats

- The interpreter uses a tree-walk, which isn't the most efficient way to
  interpret. Keep that in mind if you ever intended on using While for kernel
  programming.
- Be careful when copying examples from the handbook over into a text file, as
  some symbols (particularly `-`) are subtly different. The PDF sometimes uses
  − (`Minus Sign (U+2212)`) whereas your keyboard (and the interpreter) use
  \- (`Hyphen-Minus (U+002D)`).
- The interpreter does not yet support arrays, nor does it have a robust enough
  type system to support them.
