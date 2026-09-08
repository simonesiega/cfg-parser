# Grammar specification

[← Documentation hub](../README.md) · [CLI usage](usage.md) · [Architecture](architecture.md)

This document is the canonical specification of the arithmetic language accepted by CFG Parser.

It defines the token vocabulary, grammar, precedence, associativity, implicit-multiplication rules, and arithmetic semantics. Implementation details belong in [Architecture](architecture.md).

## Scope

CFG Parser supports:

- unsigned decimal literals;
- unary negation;
- `+`, `-`, `*`, and `/`;
- exponentiation with `^`;
- n-th roots with `$`;
- nested parentheses;
- implicit multiplication;
- a required `=` formula terminator.

Values and intermediate results use Rust `f64` arithmetic.

The language does not define variables, assignment, functions, scientific notation, or unary `+`.

## Notation

| Form | Meaning |
| --- | --- |
| `F`, `E`, `P`, `U`, `B` | Non-terminal |
| `"+"` | Literal terminal token |
| `ε` | Empty production |
| `→` | Produces |

`E'`, `P'`, and `U'` are continuation productions.

## Lexical tokens

| Input | Token |
| --- | --- |
| decimal literal | `Number(f64)` |
| `+` | `Plus` |
| `-` | `Minus` |
| `*` | `Multiply` |
| `/` | `Divide` |
| `^` | `Caret` |
| `$` | `Dollar` |
| `(` | `LeftParen` |
| `)` | `RightParen` |
| `=` | `Equals` |

### Numbers

Number literals are unsigned and use ASCII digits with at most one decimal point.

Accepted forms include:

```text
0
42
3.14
.5
1.
```

Malformed forms such as `.` and `1..2` are rejected.

Scientific notation is not recognized because `e` and `E` are not language tokens.

A sign is not part of a number token. For example:

```text
-2
```

is tokenized as:

```text
Minus, Number(2)
```

and handled by the unary-negation production.

### Whitespace

ASCII whitespace may appear between tokens.

```text
2 + 3 =
2+3=
```

represent the same expression.

Whitespace can also separate two number tokens:

```text
2 3 =
```

which matters because adjacent numbers are accepted as implicit multiplication. `23 =`, by contrast, contains one number token.

## Formal grammar

```text
F  → E "="

E  → P E'
E' → "+" P E'
   | "-" P E'
   | ε

P  → U P'
P' → "*" U P'
   | "/" U P'
   | ImplicitMult U P'
   | ε

U  → B U'
U' → "^" U
   | "$" U
   | ε

B  → "-" B
   | unsigned_number
   | "(" E ")"
```

`ImplicitMult` is not a lexical token. It represents the parser-side adjacency rule defined in [Implicit multiplication](#implicit-multiplication).

## Precedence and associativity

From highest binding to lowest:

| Level | Syntax | Associativity |
| ---: | --- | --- |
| 1 | Base and unary negation (`B`) | Unary recursion |
| 2 | `^`, `$` | Right |
| 3 | `*`, `/`, implicit multiplication | Left |
| 4 | `+`, `-` | Left |

Parentheses recursively evaluate an inner `E` and therefore override the normal precedence hierarchy.

### Unary negation

Unary negation belongs to `B`, so it binds before exponentiation and roots.

```text
-2 ^ 2 =
```

is interpreted as:

```text
(-2) ^ 2
```

and evaluates to `4`.

To negate the result of a power:

```text
-(2 ^ 2) =
```

which evaluates to `-4`.

This differs from some calculators and mathematical conventions, so explicit parentheses are recommended when clarity matters.

Unary `+` is not part of the grammar.

### Left-associative operators

Addition, subtraction, multiplication, division, and implicit multiplication evaluate from left to right within their precedence level.

```text
20 - 5 - 3
```

groups as:

```text
(20 - 5) - 3
```

and:

```text
100 / 10 / 2
```

groups as:

```text
(100 / 10) / 2
```

### Powers and roots

`^` and `$` share one precedence level and associate to the right because `U'` recursively parses another `U`.

```text
2 ^ 3 ^ 2
```

groups as:

```text
2 ^ (3 ^ 2)
```

Mixed chains follow the same rule:

```text
16 $ 2 ^ 2  → 16 $ (2 ^ 2)
4 ^ 2 $ 2   → 4 ^ (2 $ 2)
```

Neither `^` nor `$` has priority over the other.

## Formula termination

A complete input must match:

```text
F → E "="
```

`=` terminates the formula. It is not an assignment operator and does not participate in the calculation.

Valid:

```text
2 + 3 =
```

Invalid:

```text
2 + 3
2 + 3 = 5
```

The first is missing the terminator. The second contains trailing tokens after it.

Because parenthesized expressions contain `E`, not `F`, `=` is only used to terminate the outer formula.

## Implicit multiplication

Implicit multiplication is recognized at the product layer from adjacent token classes.

| Previous token | Next token | Interpretation |
| --- | --- | --- |
| number | number | `number * number` |
| number | `(` | `number * (` |
| `)` | number | `) * number` |
| `)` | `(` | `) * (` |

Examples:

```text
2 3 =              → 2 * 3
2(3 + 4) =         → 2 * (3 + 4)
(1 + 2)3 =         → (1 + 2) * 3
(1 + 2)(3 + 4) =   → (1 + 2) * (3 + 4)
```

No synthetic multiplication token is inserted by the tokenizer. The parser detects the adjacency while evaluating the product layer.

Adjacent numbers are valid in the current language, although explicit `*` is recommended between standalone numbers for readability.

## Operator semantics

### Basic arithmetic

The four basic binary operators use normal `f64` arithmetic:

```text
a + b
a - b
a * b
a / b
```

Division by zero is rejected.

### Exponentiation

```text
a ^ b
```

is evaluated with `f64::powf`.

Results that become `NaN` or infinity are rejected.

For example:

```text
-4 ^ .5 =
```

produces a mathematical error because the parsed base is `-4` and the resulting real-valued `f64` operation is invalid.

### N-th roots

```text
a $ b
```

means the `b`-th root of `a`.

For non-negative bases it is computed as:

```text
a^(1 / b)
```

Examples:

```text
27 $ 3 =  → 3
16 $ 2 =  → 4
```

Negative bases accept odd integer roots:

```text
-8 $ 3 =  → -2
```

Root evaluation rejects:

- a zero index;
- a fractional index for a negative base;
- an even integer index for a negative base;
- a resulting `NaN` or infinity.

A zero root index uses the evaluator's division-by-zero error path.

## Numeric model

All literals and intermediate values use IEEE 754 `f64`.

The evaluator checks several invalid numeric conditions during arithmetic operations, including:

- division by zero;
- infinite arithmetic results;
- subnormal results where the operation is checked;
- invalid or non-finite exponentiation;
- invalid or non-finite root results.

Normal floating-point rounding still applies. CFG Parser does not provide arbitrary-precision or exact decimal arithmetic.

CLI output formatting is documented in [CLI usage](usage.md).

## Accepted examples

| Expression | Grouping or meaning | Result |
| --- | --- | ---: |
| `2 + 3 * 4 =` | `2 + (3 * 4)` | `14` |
| `8 / 4 / 2 =` | `(8 / 4) / 2` | `1` |
| `-2 ^ 2 =` | `(-2) ^ 2` | `4` |
| `2 ^ 3 ^ 2 =` | `2 ^ (3 ^ 2)` | `512` |
| `16 $ 2 ^ 2 =` | `16 $ (2 ^ 2)` | `2` |
| `2 3 =` | `2 * 3` | `6` |
| `2(3 + 1) =` | `2 * (3 + 1)` | `8` |
| `(1 + 2)3 =` | `(1 + 2) * 3` | `9` |
| `(1 + 2)(3 + 4) =` | `(1 + 2) * (3 + 4)` | `21` |
| `27 $ 3 =` | Cube root | `3` |
| `-8 $ 3 =` | Real cube root of a negative base | `-2` |

## Rejected examples

| Expression | Reason |
| --- | --- |
| `+2 =` | Unary `+` is not defined. |
| `2 + =` | Missing right operand. |
| `2 * (3 + 4 =` | Missing closing parenthesis. |
| `2 ^ =` | Missing exponent. |
| `27 $ 0 =` | Root index is zero. |
| `-16 $ 2 =` | Even root of a negative base. |
| `-8 $ 2.5 =` | Fractional root index for a negative base. |
| `-4 ^ .5 =` | Invalid real-valued exponentiation. |
| `2 + 3` | Missing final `=`. |
| `2 + 3 = 5` | Trailing token after `=`. |
| `1e3 =` | Scientific notation is not tokenized. |
| `1..2 =` | Multiple decimal points. |

## Implementation mapping

The grammar maps directly to the recursive-descent parser:

| Production | Implementation |
| --- | --- |
| `F` | `evaluate` |
| `E` | `evaluate_e`, `evaluate_e_prime` |
| `P` | `evaluate_p`, `evaluate_p_prime` |
| `U` | `evaluate_u`, `evaluate_u_prime` |
| `B` | `evaluate_b` |

For the tokenizer, parser pipeline, direct-evaluation design, errors, and test boundaries, see [Architecture](architecture.md).
