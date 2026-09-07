use std::env;

/// # Custom logging macros for debug and release
///
/// This block defines a set of logging macros (`trace_log!`, `debug_log!`, `info_log!`, `warn_log!`, `error_log!`)
/// that are active only in `debug` mode (when `cfg(debug_assertions)` is enabled).
///
/// In `release` mode, all these macros become no-ops,
/// reducing logging overhead in production.
// MACROS ACTIVE IN DEBUG MODE //
#[cfg(debug_assertions)]
#[allow(unused)]
macro_rules! trace_log { ($($arg:tt)*) => { log::trace!($($arg)*); }; }
#[cfg(debug_assertions)]
#[allow(unused)]
macro_rules! debug_log { ($($arg:tt)*) => { log::debug!($($arg)*); }; }
#[cfg(debug_assertions)]
#[allow(unused)]
macro_rules! info_log { ($($arg:tt)*) => { log::info!($($arg)*); }; }
#[cfg(debug_assertions)]
#[allow(unused)]
macro_rules! warn_log { ($($arg:tt)*) => { log::warn!($($arg)*); }; }
#[cfg(debug_assertions)]
#[allow(unused)]
macro_rules! error_log { ($($arg:tt)*) => { log::error!($($arg)*); }; }

// NO-OP VERSIONS IN RELEASE MODE
#[cfg(not(debug_assertions))]
#[allow(unused_macros)]
macro_rules! trace_log {
    ($($arg:tt)*) => {};
}
#[cfg(not(debug_assertions))]
#[allow(unused_macros)]
macro_rules! debug_log {
    ($($arg:tt)*) => {};
}
#[cfg(not(debug_assertions))]
macro_rules! info_log {
    ($($arg:tt)*) => {};
}
#[cfg(not(debug_assertions))]
#[allow(unused_macros)]
macro_rules! warn_log {
    ($($arg:tt)*) => {};
}
#[cfg(not(debug_assertions))]
macro_rules! error_log {
    ($($arg:tt)*) => {};
}

/// # Enum `Token`
///
/// Represents recognized lexical tokens.
/// Each variant maps to a symbol type in the arithmetic language:
/// - `Number(f64)`: a decimal number.
/// - `Plus`, `Minus`, `Multiply`, `Divide`: arithmetic operators.
/// - `Caret`, `Dollar`: exponentiation and root symbols.
/// - `LeftParen`, `RightParen`: round parentheses.
/// - `Equals`: expression terminator or assignment symbol.
///
/// Derives:
/// - `Debug`: for readable printing during debug/logging.
/// - `Clone` and `Copy`: to duplicate tokens, since they are lightweight immutable types.
/// - `PartialEq`: to compare tokens (e.g., in parser logic).
#[derive(Debug, Clone, Copy, PartialEq)]
enum Token {
    /// Real number (e.g. 3.14, 42.0)
    Number(f64),

    /// Addition operator: '+'
    Plus,

    /// Subtraction operator: '-'
    Minus,

    /// Multiplication operator: '*'
    Multiply,

    /// Division operator: '/'
    Divide,

    /// Exponentiation symbol: '^'
    Caret,

    /// n-th root symbol: '$'
    Dollar,

    /// Opening parenthesis: '('
    LeftParen,

    /// Closing parenthesis: ')'
    RightParen,

    /// End-of-expression symbol: '='
    Equals,
}

impl Token {
    /// Creates a token from a specific character.
    ///
    /// Returns `Some(Token)` if the character maps to a valid token,
    /// otherwise `None`.
    ///
    /// # Parameters
    /// - `c`: The character to interpret as a token.
    ///
    /// # Example
    /// ```
    /// assert_eq!(Token::from_char('+'), Some(Token::Plus));
    /// assert_eq!(Token::from_char('x'), None);
    /// ```
    #[inline] // Suggests inlining this function for efficiency.
    fn from_char(c: char) -> Option<Self> {
        match c {
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Multiply),
            '/' => Some(Token::Divide),
            '^' => Some(Token::Caret),
            '$' => Some(Token::Dollar),
            '(' => Some(Token::LeftParen),
            ')' => Some(Token::RightParen),
            '=' => Some(Token::Equals),
            _ => None, // character not recognized as a token
        }
    }

    /// Checks whether the token is a binary (mathematical) operator.
    ///
    /// # Example
    /// ```
    /// assert!(Token::Plus.is_operator());
    /// assert!(!Token::LeftParen.is_operator());
    /// ```
    #[inline]
    #[allow(unused)]
    fn is_operator(&self) -> bool {
        matches!(
            self,
            Token::Plus | Token::Minus | Token::Multiply | Token::Divide
        )
    }
}

/// Error types that can occur during mathematical computations.
///
/// Used by the arithmetic expression evaluator to report
/// errors such as division by zero or computational limits.
///
/// Derives:
/// - `Debug`: allows error formatting for logs/debugging.
/// - `PartialEq`: allows comparing errors in tests or control flow.
#[derive(Debug, PartialEq)]
#[allow(unused)]
enum MathError {
    /// Division by zero.
    DivisionByZero,

    /// Result exceeded the upper representable limits.
    OverflowError,

    /// Result fell below the lower representable limits.
    UnderflowError,

    /// The expression contains too many elements or nested levels.
    // Not currently implemented
    ExpressionTooComplex,

    /// Exponentiation with invalid base or exponent.
    InvalidExponentiation { base: f64, exponent: f64 },

    /// Root of a negative number with a fractional index.
    NegativeRoot { base: f64, root: f64 },

    /// Even-index root of a negative number (undefined over reals).
    EvenRootOfNegative { base: f64, root: f64 },

    /// Root with invalid base or index.
    InvalidRoot { base: f64, root: f64 },
}

/// Error types that can occur during tokenization or parsing.
/// Used to signal syntax errors or invalid input.
///
/// Derives:
/// - `Debug`: allows error formatting for logs/debugging.
/// - `PartialEq`: allows comparing errors in tests or control flow.
#[derive(Debug, PartialEq)]
#[allow(unused)]
enum TokenError {
    /// Malformed or invalid number (e.g. "1..2").
    InvalidNumber(String),

    /// Input terminates unexpectedly (e.g. unclosed parenthesis).
    UnexpectedEnd,

    /// Syntactically invalid expression.
    InvalidExpression(String),

    /// Unrecognized operator (e.g. '%', '^', etc.).
    InvalidOperator(char),

    /// Missing matching parenthesis; includes found char and position.
    UnmatchedParenthesis { found: char, position: usize },

    /// Unexpected token found at the current parsing position.
    UnexpectedToken(Token),

    /// Generic syntax error with message.
    // Not currently implemented
    SyntaxError(String),
}

/// `Display` implementation for `MathError`.
///
/// Converts each error into a human-readable string,
/// suitable for user output or logging.
///
/// Each branch also logs through `error_log!`,
/// enabled only under `debug_assertions`.
impl std::fmt::Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MathError::DivisionByZero => {
                error_log!("Error: division by zero");
                write!(f, "Math error: division by zero")
            }
            MathError::OverflowError => {
                error_log!("Error: numeric overflow");
                write!(f, "Math error: numeric overflow")
            }
            MathError::UnderflowError => {
                error_log!("Error: numeric underflow");
                write!(f, "Math error: numeric underflow")
            }
            MathError::ExpressionTooComplex => {
                error_log!("Error: expression too complex");
                write!(f, "Error: expression too complex")
            }
            MathError::InvalidExponentiation { base, exponent } => {
                error_log!(
                    "Error: invalid exponentiation (base: {}, exponent: {})",
                    base,
                    exponent
                );
                write!(f, "Error: invalid exponentiation ({} ^ {})", base, exponent)
            }
            MathError::NegativeRoot { base, root } => {
                error_log!(
                    "Error: fractional-index root of a negative number (base: {}, index: {})",
                    base,
                    root
                );
                write!(
                    f,
                    "Error: fractional root of a negative number ({} $ {})",
                    base, root
                )
            }
            MathError::EvenRootOfNegative { base, root } => {
                error_log!(
                    "Error: even-index root of a negative number (base: {}, index: {})",
                    base,
                    root
                );
                write!(
                    f,
                    "Error: even-index root of a negative number ({} $ {})",
                    base, root
                )
            }
            MathError::InvalidRoot { base, root } => {
                error_log!("Error: invalid root (base: {}, index: {})", base, root);
                write!(f, "Error: invalid root ({} $ {})", base, root)
            }
        }
    }
}

/// `Display` implementation for `TokenError`.
///
/// Converts each error into a human-readable string,
/// suitable for user output or logging.
///
/// Each branch also logs through `error_log!`,
/// enabled only under `debug_assertions`.
impl std::fmt::Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TokenError::InvalidNumber(msg) => {
                error_log!("Invalid number: {}", msg);
                write!(f, "Invalid number: {}", msg)
            }
            TokenError::UnmatchedParenthesis { found, position } => {
                error_log!("Missing parenthesis '{}' at position {}", found, position);
                write!(f, "Error: missing '{}' at position {}", found, position)
            }
            TokenError::UnexpectedEnd => {
                error_log!("Error: unexpected end of expression");
                write!(f, "Error: expression ended unexpectedly")
            }
            TokenError::InvalidExpression(msg) => {
                error_log!("Error: invalid expression ({})", msg);
                write!(f, "Error: invalid expression - {}", msg)
            }
            TokenError::InvalidOperator(op) => {
                error_log!("Invalid operator: '{}'", op);
                write!(f, "Error: invalid operator '{}'", op)
            }
            TokenError::UnexpectedToken(token) => {
                error_log!("Unexpected token: {:?}", token);
                write!(f, "Error: unexpected token {:?}", token)
            }
            TokenError::SyntaxError(msg) => {
                error_log!("Syntax error: {}", msg);
                write!(f, "Syntax error: {}", msg)
            }
        }
    }
}

/// `Error` implementation for `MathError`.
///
/// Allows `MathError` to be handled as a standard error,
/// for example with `?`.
impl std::error::Error for MathError {}

/// `Error` implementation for `TokenError`.
///
/// Allows `TokenError` to be handled as a standard error,
/// for example with `?`.
impl std::error::Error for TokenError {}

/// Represents a generic computation error.
///
/// Unifies mathematical errors (`MathError`)
/// and tokenization/parsing errors (`TokenError`).
///
/// - `Debug`, `PartialEq`.
#[derive(Debug, PartialEq)]
enum CalcError {
    // Mathematical error
    Math(MathError),
    // Parsing error
    Token(TokenError),
}

/// Automatic conversion from `MathError` to `CalcError`.
/// Allows using `?` in functions that return `CalcResult`.
impl From<MathError> for CalcError {
    fn from(e: MathError) -> Self {
        CalcError::Math(e)
    }
}

/// Automatic conversion from `TokenError` to `CalcError`.
/// Allows using `?` in functions that return `CalcResult`.
impl From<TokenError> for CalcError {
    fn from(e: TokenError) -> Self {
        CalcError::Token(e)
    }
}

/// `Display` implementation for `CalcError`.
///
/// Produces a readable message by combining `MathError` and `TokenError`.
/// Detailed messages are delegated to each type's `Display` implementation.
impl std::fmt::Display for CalcError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CalcError::Math(e) => write!(f, "Math error: {}", e),
            CalcError::Token(e) => write!(f, "Parsing error: {}", e),
        }
    }
}

/// `Error` implementation for `CalcError`.
///
/// Allows `CalcError` to be handled as a standard error,
/// for example with `?`.
impl std::error::Error for CalcError {}

/// Alias for the result type returned by computation functions.
///
/// - `Ok(f64)`: numeric computation result.
/// - `Err(CalcError)`: error that can be:
///   - `MathError`: arithmetic errors (e.g. division by zero, overflow).
///   - `TokenError`: syntax or expression parsing errors.
type CalcResult = Result<f64, CalcError>;

/// Structure responsible for lexical analysis of a mathematical expression.
///
/// Splits the input string into a sequence of recognized tokens.
/// Tracks the current scanning position.
/// - `'a`: lifetime of the input string reference.
/// - Uses an immutable reference (`&'a str`) to avoid unnecessary string copies.
/// - `position` tracks the current index while scanning characters.
struct Tokenizer<'a> {
    /// Immutable input string slice containing the expression to analyze.
    input: &'a str,
    /// Current position in the input, used to track progress during tokenization.
    position: usize,
}

impl<'a> Tokenizer<'a> {
    /// Creates a new `Tokenizer` instance for the given input string.
    ///
    /// # Parameters
    /// - `input`: reference to the string to analyze.
    ///
    /// # Returns
    /// - Initialized `Tokenizer` instance with starting position set to zero.
    fn new(input: &'a str) -> Self {
        info_log!("Tokenizer created. Input: '{}'", input);
        Self { input, position: 0 }
    }

    /// Parses the input string and produces a token sequence.
    ///
    /// # Returns
    /// - `Ok(Vec<Token>)` on success.
    /// - `Err(TokenError)` if a syntax or invalid-symbol error is detected.
    fn tokenize(&mut self) -> Result<Vec<Token>, TokenError> {
        info_log!("Starting tokenization");
        let mut tokens = Vec::new();

        // Scans each character until reaching the end of input.
        while self.position < self.input.len() {
            let c = self.current_char();

            match c {
                // Ignores whitespace.
                c if c.is_whitespace() => self.advance(),

                // Handles numeric sequences, including decimals.
                c if c.is_ascii_digit() || c == '.' => {
                    let token = self.parse_number()?;
                    info_log!("Number token found: {:?}", token);
                    tokens.push(token);
                }

                // Handles symbols and operators.
                c => {
                    // Handles recognized tokens.
                    if let Some(token) = Token::from_char(c) {
                        info_log!("Symbol token found: {:?}", token);
                        tokens.push(token);
                        self.advance();
                    }
                    // Handles unrecognized tokens using InvalidOperator, where c is the unknown character.
                    else {
                        return Err(TokenError::InvalidOperator(c));
                    }
                }
            }
        }

        // Tokenization completed: return OK and the token vector to parse.
        info_log!("Tokenization completed: {:?}", tokens);
        Ok(tokens)
    }

    /// Parses and builds a numeric token from the current position.
    ///
    /// Supports integer and decimal numbers. Multiple decimal points are not allowed.
    ///
    /// # Returns
    /// - `Ok(Token::Number(f64))` if parsing succeeds.
    /// - `Err(TokenError::InvalidNumber)` for malformed numbers.
    fn parse_number(&mut self) -> Result<Token, TokenError> {
        let start = self.position;
        let mut has_decimal = false;

        // Continues reading while characters are part of the number.
        while self.position < self.input.len() {
            match self.current_char() {
                c if c.is_ascii_digit() => self.advance(),

                // Accepts only one decimal point.
                '.' if !has_decimal => {
                    has_decimal = true;
                    self.advance();
                }

                // Rejects numbers with multiple decimal points.
                // If a second '.' appears after the number is already marked as decimal, return an error.
                // Invalid example: "2..3"
                '.' => {
                    return Err(TokenError::InvalidNumber(
                        "Number with multiple decimal points".into(),
                    ));
                }

                // Stops reading at the first non-numeric character.
                _ => break,
            }
        }

        // Extracts the substring representing a number from start to current position.
        let number_str = &self.input[start..self.position];

        // Attempts to parse the substring into an `f64` numeric value.
        // On success, returns `Token::Number(n)`.
        // On parse failure, returns `TokenError::InvalidNumber` with the invalid string.
        match number_str.parse::<f64>() {
            Ok(n) => Ok(Token::Number(n)),
            Err(_) => Err(TokenError::InvalidNumber(number_str.to_string())),
        }
    }

    /// Returns the current input character at the current position.
    /// Uses `chars().next().unwrap()` to access the first remaining character,
    /// assuming the position is always valid and within input bounds.
    fn current_char(&self) -> char {
        self.input[self.position..].chars().next().unwrap()
    }

    /// Advances the current position by one, moving to the next input character.
    /// Positioning is character-index based and assumes `current_char()` was already evaluated.
    fn advance(&mut self) {
        self.position += 1;
    }
}

/// Parser for mathematical expressions based on a token sequence.
/// Handles syntax analysis and expression evaluation according to operator precedence.
struct MathExpressionParser {
    /// Sequence of tokens generated by the tokenizer.
    tokens: Vec<Token>,
    /// Current position within the token vector.
    position: usize,
}

impl MathExpressionParser {
    /// Builds a new parser from a token sequence.
    ///
    /// # Parameters
    /// - `tokens`: Vector of pre-tokenized input to evaluate.
    ///
    /// # Returns
    /// An initialized `MathExpressionParser` instance with starting position set to zero.
    fn new(tokens: Vec<Token>) -> Self {
        info_log!("Parser initialized with tokens: {:?}", tokens);
        Self {
            tokens,
            position: 0,
        }
    }

    /// Evaluates a complete arithmetic expression according to the formal grammar.
    ///
    /// This method is the main entry point for parsing and evaluating
    /// a formula, following the grammar rule:
    /// ```
    /// F → E "="
    /// ```
    ///
    /// # Behavior
    /// - Evaluates the expression through `evaluate_e()`.
    /// - Verifies the presence of `=` at the end.
    /// - Returns the computed result if valid, otherwise returns an error.
    ///
    /// # Returns
    /// - `Ok(f64)` if the expression is valid and correctly terminated with `=`
    /// - `Err(CalcError)` in case of syntax (unexpected token, premature end) or semantic errors
    ///
    /// # Examples
    /// ```
    /// let mut parser = Parser::new("2 + 3 * 4 =");
    /// let result = parser.evaluate();
    /// assert_eq!(result.unwrap(), 14.0);
    /// ```
    ///
    /// ```
    /// let mut parser = Parser::new("2 + =");
    /// let result = parser.evaluate();
    /// assert!(result.is_err()); // Error: missing term after '+'
    /// ```
    ///
    /// # Notes
    /// - The `=` symbol is required as the final delimiter, but it is not part of the calculation.
    /// - Internal logs help trace evaluation state.
    fn evaluate(&mut self) -> CalcResult {
        info_log!("Starting evaluation");
        let result = self.evaluate_e()?; // Parses and evaluates a full expression.

        // Requires '=' after the expression, consumes it,
        // then verifies no trailing tokens remain.
        match self.next() {
            Some(Token::Equals) => match self.peek() {
                Some(token) => {
                    error_log!("Unexpected trailing token after '=': {:?}", token);
                    Err(TokenError::UnexpectedToken(*token).into())
                }
                None => {
                    info_log!("Evaluation completed successfully");
                    Ok(result)
                }
            },
            Some(token) => {
                // Error: unexpected token after end of expression.
                error_log!("Unexpected token after evaluation: {:?}", token);
                Err(TokenError::UnexpectedToken(token).into())
            }
            None => {
                // Error: expression ended without an explicit '='.
                error_log!("Incomplete expression at end");
                Err(TokenError::UnexpectedEnd.into())
            }
        }
    }

    /// Evaluates an arithmetic expression that may contain addition and subtraction between terms.
    ///
    /// This method implements the grammar rule:
    /// ```
    /// E → P E'
    /// ```
    ///
    /// # Behavior
    /// - Evaluates the first term `P` via `evaluate_p()`.
    /// - Then passes the partial result to `evaluate_e_prime()` to process
    ///   any additions or subtractions from production `E'`.
    /// - Evaluation stops when no `+` or `-` operators remain.
    ///
    /// # Returns
    /// - `Ok(f64)` with the evaluated expression result.
    /// - `Err(CalcError)` on syntax or semantic errors.
    ///
    /// # Examples
    /// ```
    /// let mut parser = Parser::new("3 + 2 =");
    /// assert_eq!(parser.evaluate_e().unwrap(), 5.0);
    /// ```
    ///
    /// ```
    /// let mut parser = Parser::new("7 - 4 =");
    /// assert_eq!(parser.evaluate_e().unwrap(), 3.0);
    /// ```
    fn evaluate_e(&mut self) -> CalcResult {
        let result = self.evaluate_p()?;
        self.evaluate_e_prime(result)
    }

    /// Evaluates the recursive part of an expression (`E'`) that handles addition and subtraction.
    ///
    /// This method implements the grammar rule:
    /// ```
    /// E' → "+" P E'
    ///     | "−" P E'
    ///     | ε
    /// ```
    ///
    /// # Parameters
    /// - `acc`: Accumulated value so far, result of evaluating `P` in `E → P E'`.
    ///
    /// # Behavior
    /// - In a loop, checks whether the current token is `+` or `-`.
    /// - If `+`, evaluates the next `P` term and adds it to the accumulator.
    /// - If `-`, evaluates the next `P` term and subtracts it from the accumulator.
    /// - In both cases, checks numeric overflow via `check_overflow()`.
    /// - If the next token is not an operator, returns the accumulated value.
    ///
    /// # Returns
    /// - `Ok(f64)` with the updated expression result.
    /// - `Err(CalcError)` on arithmetic errors (e.g. overflow).
    ///
    /// # Example
    /// ```
    /// let mut parser = Parser::new("5 + 3 - 2 =");
    /// assert_eq!(parser.evaluate_e_prime(5.0).unwrap(), 6.0);
    /// ```
    fn evaluate_e_prime(&mut self, mut acc: f64) -> CalcResult {
        loop {
            match self.peek() {
                // Consumes the token in both cases.
                Some(Token::Plus) => {
                    self.advance();
                    let rhs = self.evaluate_p()?; // Right-Hand Side

                    info_log!("Operation: {} + {}", acc, rhs);
                    acc = self.check_overflow(acc + rhs)?;
                }
                Some(Token::Minus) => {
                    self.advance();
                    let rhs = self.evaluate_p()?; // Right-Hand Side

                    info_log!("Operation: {} - {}", acc, rhs);
                    acc = self.check_overflow(acc - rhs)?;
                }
                _ => break,
            }
        }
        // Returns the accumulated value.
        Ok(acc)
    }

    /// Evaluates a product-level part of the expression, which may include:
    /// - Explicit multiplication (`*`) and division (`/`) operations
    /// - Implicit multiplication (e.g. `2(3+4)` -> `2 * (3+4)`)
    ///
    /// This method implements the grammar rule:
    /// ```
    /// P → U P'
    /// ```
    ///
    /// # Behavior
    /// - Calls `evaluate_u()` to evaluate the first expression unit.
    /// - Passes the result to `evaluate_p_prime()` to process subsequent operations.
    ///
    /// # Returns
    /// - `Ok(f64)` with the computed product value.
    /// - `Err(CalcError)` on math or syntax errors.
    ///
    /// # Examples
    /// ```
    /// let mut parser = Parser::new("2 * 3 =");
    /// assert_eq!(parser.evaluate_p().unwrap(), 6.0);
    /// ```
    ///
    /// ```
    /// let mut parser = Parser::new("4(1 + 2) =");
    /// assert_eq!(parser.evaluate_p().unwrap(), 12.0);  // implicit multiplication
    /// ```
    fn evaluate_p(&mut self) -> CalcResult {
        let result = self.evaluate_u()?;
        self.evaluate_p_prime(result)
    }

    /// Evaluates subsequent product operations, including:
    /// - Explicit multiplication (`*`)
    /// - Division (`/`)
    /// - Implicit multiplication (e.g. `2(3 + 1)` -> `2 * (3 + 1)`)
    ///
    /// This method implements the grammar rule:
    /// ```
    /// P' → "*" U P'
    ///     | "/" U P'
    ///     | ImplicitMult U P'
    ///     | ε
    /// ```
    ///
    /// # Behavior
    /// - Iterates through all tokens that can continue `P`.
    /// - For `*` or `/`, evaluates the right-hand side (`U`) and applies the operation to `acc`.
    /// - If a number or opening parenthesis appears immediately after a valid term (`acc`), applies
    ///   the implicit multiplication rule.
    /// - Stops at the first token that is not a valid continuation.
    ///
    /// # Handled errors
    /// - `MathError::DivisionByZero` if division by zero is attempted.
    /// - `MathError::OverflowError` or `MathError::UnderflowError` if result exceeds numeric limits.
    ///
    /// # Returns
    /// - `Ok(f64)` with the updated value.
    /// - `Err(CalcError)` on semantic or mathematical errors.
    ///
    /// # Examples
    /// ```
    /// let mut parser = Parser::new("4 * 2 =");
    /// assert_eq!(parser.evaluate_p_prime(4.0).unwrap(), 8.0);
    /// ```
    ///
    /// ```
    /// let mut parser = Parser::new("5(2 + 1) =");
    /// assert_eq!(parser.evaluate_p_prime(5.0).unwrap(), 15.0);  // implicit multiplication
    /// ```
    fn evaluate_p_prime(&mut self, mut acc: f64) -> CalcResult {
        loop {
            match self.peek() {
                // Consumes the token in all cases.
                // Explicit multiplication handling.
                Some(Token::Multiply) => {
                    self.advance();
                    let rhs = self.evaluate_u()?; // Right-Hand Side

                    info_log!("Multiplication: {} * {}", acc, rhs);
                    acc = self.check_overflow(acc * rhs)?;
                }
                // Explicit division handling.
                Some(Token::Divide) => {
                    self.advance();
                    let rhs = self.evaluate_u()?; // Right-Hand Side
                    // n / 0 -> Error
                    if rhs == 0.0 {
                        return Err(MathError::DivisionByZero.into());
                    }

                    info_log!("Division: {} / {}", acc, rhs);
                    acc = self.check_overflow(acc / rhs)?;
                }
                // Implicit multiplication: e.g. `2(3 + 4)` or `4 5`
                Some(Token::Number(_)) | Some(Token::LeftParen) => {
                    if self.previous_token_is_paren_or_number()
                        && self.can_apply_implicit_multiplication()
                    {
                        let rhs = self.evaluate_u()?; // Right-Hand Side

                        info_log!("Implicit multiplication: {} * {}", acc, rhs);
                        acc = self.check_overflow(acc * rhs)?;
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
        Ok(acc)
    }

    /// Checks whether the previous token is a number or a closing parenthesis.
    ///
    /// This method is used to determine whether implicit multiplication
    /// can be applied. Implicit multiplication occurs, for example,
    /// in expressions like `2(3 + 1)` or `4 5`, where the `*`
    /// operator is omitted.
    ///
    /// # Returns
    /// - `true` if the previous token is `Token::Number(_)` or `Token::RightParen`.
    /// - `false` otherwise.
    fn previous_token_is_paren_or_number(&self) -> bool {
        matches!(
            self.tokens.get(self.position.wrapping_sub(1)),
            Some(Token::Number(_)) | Some(Token::RightParen)
        )
    }

    /// Checks whether the current token can represent a valid term
    /// for implicit multiplication.
    ///
    /// This method is typically called immediately after `previous_token_is_paren_or_number`
    /// to decide whether to apply implicit multiplication between adjacent elements.
    ///
    /// # Returns
    /// - `true` if the current token is `Token::Number(_)` or `Token::LeftParen`.
    /// - `false` otherwise.
    fn can_apply_implicit_multiplication(&self) -> bool {
        matches!(self.peek(), Some(Token::Number(_)) | Some(Token::LeftParen))
    }

    /// Evaluates a unit of the arithmetic expression, which may include exponents or roots.
    ///
    /// This method implements the grammar rule:
    /// ```
    /// U → B U'
    /// ```
    ///
    /// # Behavior
    /// - Evaluates the base first via `evaluate_b()`.
    /// - Then applies optional exponent/root operations via `evaluate_u_prime(base)`.
    ///
    /// # Returns
    /// - `Ok(f64)` with the computed unit value.
    /// - `Err(CalcError)` for syntax or math errors (such as root of a negative number or overflow).
    ///
    /// # Examples
    /// ```
    /// let mut parser = Parser::new("2 ^ 3 =");
    /// assert_eq!(parser.evaluate_u().unwrap(), 8.0);
    ///
    /// let mut parser = Parser::new("27 $ 3 =");  // Cube root
    /// assert_eq!(parser.evaluate_u().unwrap(), 3.0);
    /// ```
    fn evaluate_u(&mut self) -> CalcResult {
        let base = self.evaluate_b()?;
        self.evaluate_u_prime(base)
    }

    /// Evaluates exponentiation or root operators applied to the already-computed base.
    ///
    /// This method implements the grammar rule:
    /// ```
    /// U' → "^" U
    ///     | "$" U
    ///     | ε
    /// ```
    ///
    /// # Behavior
    /// - If the current token is `^`, recursively evaluates the right-hand side and applies
    ///   exponentiation (`base ^ exponent`).
    /// - If the current token is `$`, recursively evaluates the right-hand side and applies
    ///   the root (`base $ index` = `acc` root with index `rhs`).
    /// - For any non-matching token, returns the base unchanged (epsilon production).
    ///
    /// # Validations and errors
    /// - Uses `evaluate_exponentiation` for powers, including validation
    ///   (e.g. negative base with fractional exponent).
    /// - Uses `evaluate_root` for roots, including checks for:
    ///   - Even-index root of a negative number -> `MathError::EvenRootOfNegative`
    ///   - Zero index -> `MathError::DivisionByZero`
    /// - Any out-of-range value is handled by `check_overflow`.
    ///
    /// # Parameters
    /// - `acc`: f64 - initial value the operator is applied to.
    ///
    /// # Returns
    /// - `Ok(f64)` with the result after optional exponentiation/root application.
    /// - `Err(CalcError)` on math errors (overflow, root of negative number, etc.).
    fn evaluate_u_prime(&mut self, mut acc: f64) -> CalcResult {
        match self.peek() {
            // In entrambi i casi consuma il token
            Some(Token::Caret) => {
                self.advance();
                let rhs = self.evaluate_u()?; // Right-Hand Side

                info_log!("Exponentiation: {} ^ {}", acc, rhs);
                acc = self.evaluate_exponentiation(acc, rhs)?;
                Ok(acc)
            }

            Some(Token::Dollar) => {
                self.advance();
                let rhs = self.evaluate_u()?; // Right-Hand Side

                info_log!("Root: {} $ {}", acc, rhs);
                acc = self.evaluate_root(acc, rhs)?;
                Ok(acc)
            }

            _ => Ok(acc),
        }
    }

    /// Computes exponentiation between two numbers, i.e. `base ^ exponent`.
    ///
    /// This method computes base raised to exponent and validates the result
    /// (`NaN` and infinite values are rejected).
    /// It also handles numeric overflow/underflow via `check_overflow`.
    ///
    /// # Parameters
    /// - `base`: f64 - base value.
    /// - `exponent`: f64 - exponent value.
    ///
    /// # Returns
    /// - `Ok(f64)` if calculation is valid and within numeric limits.
    /// - `Err(MathError)` on invalid exponentiation results, such as `NaN` or infinity.
    fn evaluate_exponentiation(&self, base: f64, exponent: f64) -> CalcResult {
        // Computes exponentiation: base raised to exponent.
        let result = base.powf(exponent);

        // If result is NaN or infinite, return an error.
        if result.is_nan() || result.is_infinite() {
            return Err(MathError::InvalidExponentiation { base, exponent }.into());
        }

        self.check_overflow(result)
    }

    /// Computes a root operation, i.e. `base $ root`.
    ///
    /// This method computes the `root`-th root of `base`.
    /// If base is negative and root is not an odd integer, returns
    /// `MathError::EvenRootOfNegative`.
    /// If base is negative and root is fractional, returns
    /// `MathError::NegativeRoot`.
    /// It also handles division-by-zero when `root == 0`.
    ///
    /// # Parameters
    /// - `base`: f64 - value to root.
    /// - `root`: f64 - root index.
    ///
    /// # Returns
    /// - `Ok(f64)` if calculation is valid and within numeric limits.
    /// - `Err(MathError)` on errors such as division by zero or even root of a negative number.
    fn evaluate_root(&self, base: f64, root: f64) -> CalcResult {
        // Checks whether the root index is zero, which would cause division by zero.
        if root == 0.0 {
            return Err(MathError::DivisionByZero.into());
        }

        // Handles negative base case.
        if base < 0.0 {
            // Fractional root index is invalid for negative bases in this evaluator.
            if root.fract() != 0.0 {
                return Err(MathError::NegativeRoot { base, root }.into());
            }

            // Even root index of a negative base is invalid.
            if (root as i64) % 2 == 0 {
                return Err(MathError::EvenRootOfNegative { base, root }.into());
            }

            // Computes root for negative base.
            let result = -(-base).powf(1.0 / root);
            return self.check_overflow(result);
        }

        // Computes root for non-negative base.
        let result = base.powf(1.0 / root);

        // If result is NaN or infinite, return an error.
        if result.is_nan() || result.is_infinite() {
            return Err(MathError::InvalidRoot { base, root }.into());
        }

        self.check_overflow(result)
    }

    /// Evaluates a "factor" in the arithmetic expression, which can be:
    /// - An unsigned number (e.g. `3.14`)
    /// - An expression preceded by unary negation (`-`)
    /// - A parenthesized expression (e.g. `(2 + 3)`)
    ///
    /// This method implements the grammar rule:
    /// ```
    /// B → "−" B
    ///    | unsigned number
    ///    | "(" E ")"
    /// ```
    ///
    /// # Behavior
    /// - If current token is a number (`Token::Number`), returns it directly.
    /// - If current token is unary minus (`Token::Minus`), evaluates the next factor and negates it.
    /// - If current token is `(`, evaluates an expression via `evaluate_e()` until `)` is found.
    /// - If an unexpected token is found (e.g. unmatched `)` or another invalid token), returns an error.
    /// - For a general invalid-token case, returns a syntax error.
    ///
    /// # Returns
    /// - `Ok(f64)` with the evaluated factor value (positive or negative, depending on case).
    /// - `Err(TokenError)` on syntax errors (unexpected token, unmatched parentheses, etc.).
    ///
    /// # Examples
    /// ```
    /// let mut parser = Parser::new("3.14 =");
    /// assert_eq!(parser.evaluate_b().unwrap(), 3.14);
    /// ```
    ///
    /// ```
    /// let mut parser = Parser::new("-2.5 =");
    /// assert_eq!(parser.evaluate_b().unwrap(), -2.5);
    /// ```
    fn evaluate_b(&mut self) -> CalcResult {
        match self.next() {
            // Number case: returns the number value.
            Some(Token::Number(n)) => Ok(n),

            // Negation case: evaluates next factor and negates it.
            Some(Token::Minus) => {
                let val = self.evaluate_b()?; // Factor negation.

                info_log!("Negation of {}", val);
                Ok(-val)
            }

            // Opening parenthesis case: evaluate inner expression.
            Some(Token::LeftParen) => {
                let result = self.evaluate_e()?; // Parses expression inside parentheses.

                match self.next() {
                    // Verifies the closing parenthesis matches the opening one.
                    Some(Token::RightParen) => Ok(result),

                    // If another token appears instead of `)`, return an error.
                    Some(_tok) => {
                        info_log!("Unexpected token instead of ')': {:?}", _tok);
                        Err(TokenError::UnmatchedParenthesis {
                            found: ')',
                            position: self.position,
                        }
                        .into())
                    }

                    // If there is no next token (missing closing parenthesis).
                    None => Err(TokenError::UnmatchedParenthesis {
                        found: '(',
                        position: self.position,
                    }
                    .into()),
                }
            }

            // Closing parenthesis without a matching opening parenthesis.
            Some(Token::RightParen) => {
                info_log!("Closing parenthesis without opening parenthesis");
                Err(TokenError::UnmatchedParenthesis {
                    found: ')',
                    position: self.position,
                }
                .into())
            }

            // Generic error case: invalid token found.
            _token => {
                info_log!("Invalid factor found: {:?}", _token);
                Err(TokenError::InvalidExpression("Invalid expression".into()).into())
            }
        }
    }

    /// Validates a value by checking overflow and underflow conditions.
    ///
    /// # Returns
    /// - `Ok(f64)` if value is neither infinite nor subnormal.
    /// - `Err(CalcError)` on overflow (infinite value) or underflow (subnormal value).
    ///
    /// This function validates computed values and returns an error for:
    /// - Overflow: computed value is infinite.
    /// - Underflow: computed value is subnormal, which may indicate precision loss or an excessively small value.
    ///
    fn check_overflow(&self, val: f64) -> Result<f64, CalcError> {
        // Infinite
        if val.is_infinite() {
            Err(MathError::OverflowError.into())
        }
        // Subnormal
        else if val.is_subnormal() {
            Err(MathError::UnderflowError.into())
        } else {
            Ok(val)
        }
    }

    /// Returns the current token without advancing the parser position.
    ///
    /// # Returns
    /// - `Some(&Token)` if a token exists at current position.
    /// - `None` if current position is outside token bounds.
    ///
    /// Allows inspecting the current token without consuming it.
    /// Useful for lookahead decisions and parser state checks.
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    /// Returns the current token and advances to the next position.
    ///
    /// # Returns
    /// - `Some(Token)` if a token exists at current position, then advances.
    /// - `None` if current position is outside token bounds.
    ///
    /// This method returns the current token and increments parser position.
    /// Useful for iterating through the token stream.
    fn next(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.position).copied(); // Since `.get()` returns `&Token`, `.copied()` is used to copy the value out of `Option`.
        // If a valid token exists
        if token.is_some() {
            self.advance();
        }
        token
    }

    /// Advances to the next token position.
    fn advance(&mut self) {
        self.position += 1;
    }
}

const DEFAULT_INPUT: &str = "(3 + 5 * (2 - 3) ^ 2) / (4 - 1) + -2 * (5 + 2) ^ 3 - 10 =";

fn resolve_input_expression() -> String {
    let args: Vec<String> = env::args().skip(1).collect();

    if !args.is_empty() {
        return args.join(" ");
    }

    if let Ok(input) = env::var("CFGPARSER_INPUT") {
        if !input.trim().is_empty() {
            return input;
        }
    }

    DEFAULT_INPUT.to_string()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    /*
    #[cfg(debug_assertions)]
    {
        env_logger::Builder::new()
            .format(|buf, record| {
                use std::io::Write;
                writeln!(buf, "[{} {}] {}", record.level(), record.target(), record.args())
            })
            .filter_level(log::LevelFilter::Debug)
            .init();
    }
    */

    let input = resolve_input_expression();
    info_log!("Input expression: {}", input);

    let mut tokenizer = Tokenizer::new(&input);

    let result = match tokenizer.tokenize() {
        Ok(tokens) => {
            let mut parser = MathExpressionParser::new(tokens);
            parser.evaluate()
        }
        Err(e) => Err(CalcError::Token(e)),
    };

    match result {
        Ok(value) => {
            println!("Result: {:.3}", value);
            Ok(())
        }
        Err(e) => {
            // println!("Error: {}", e);
            match e {
                CalcError::Math(math_err) => {
                    error_log!("Math error: {}", math_err);
                    Err(Box::new(math_err))
                }
                CalcError::Token(token_err) => {
                    error_log!("Tokenization error: {}", token_err);
                    Err(Box::new(token_err))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tokenize(input: &str) -> Result<Vec<Token>, TokenError> {
        Tokenizer::new(input).tokenize()
    }

    fn evaluate(input: &str) -> CalcResult {
        let tokens = tokenize(input)?;
        MathExpressionParser::new(tokens).evaluate()
    }

    fn assert_evaluates_to(input: &str, expected: f64) {
        let actual = evaluate(input).unwrap_or_else(|error| {
            panic!("expected {input:?} to evaluate successfully, got {error:?}")
        });
        let tolerance = f64::EPSILON * expected.abs().max(1.0) * 8.0;
        assert!(
            (actual - expected).abs() <= tolerance,
            "expected {input:?} to equal {expected}, got {actual}"
        );
    }

    #[test]
    fn maps_every_operator_character_to_a_token() {
        let cases = [
            ('+', Token::Plus),
            ('-', Token::Minus),
            ('*', Token::Multiply),
            ('/', Token::Divide),
            ('^', Token::Caret),
            ('$', Token::Dollar),
            ('(', Token::LeftParen),
            (')', Token::RightParen),
            ('=', Token::Equals),
        ];

        for (character, expected) in cases {
            assert_eq!(Token::from_char(character), Some(expected));
        }
        assert_eq!(Token::from_char('%'), None);
    }

    #[test]
    fn identifies_basic_binary_operators() {
        for token in [Token::Plus, Token::Minus, Token::Multiply, Token::Divide] {
            assert!(token.is_operator());
        }
        for token in [
            Token::Caret,
            Token::Dollar,
            Token::LeftParen,
            Token::Number(1.0),
        ] {
            assert!(!token.is_operator());
        }
    }

    #[test]
    fn tokenizes_numbers_symbols_and_ascii_whitespace() {
        assert_eq!(
            tokenize("\t.5 + 1. - 23\n= ").unwrap(),
            vec![
                Token::Number(0.5),
                Token::Plus,
                Token::Number(1.0),
                Token::Minus,
                Token::Number(23.0),
                Token::Equals,
            ]
        );
    }

    #[test]
    fn rejects_malformed_numbers() {
        assert_eq!(
            tokenize("1..2 ="),
            Err(TokenError::InvalidNumber(
                "Number with multiple decimal points".into()
            ))
        );
        assert_eq!(tokenize(". ="), Err(TokenError::InvalidNumber(".".into())));
    }

    #[test]
    fn rejects_unknown_ascii_and_unicode_characters() {
        assert_eq!(tokenize("2 % 1 ="), Err(TokenError::InvalidOperator('%')));
        assert_eq!(tokenize("π ="), Err(TokenError::InvalidOperator('π')));
    }

    #[test]
    fn evaluates_literals_decimals_and_whitespace() {
        assert_evaluates_to("42 =", 42.0);
        assert_evaluates_to(" .5 + 1.25 = ", 1.75);
        assert_evaluates_to("\n\t3\r + 4 =", 7.0);
    }

    #[test]
    fn applies_additive_and_multiplicative_precedence() {
        assert_evaluates_to("2 + 3 * 4 =", 14.0);
        assert_evaluates_to("2 * 3 + 4 =", 10.0);
        assert_evaluates_to("20 / 5 - 1 =", 3.0);
    }

    #[test]
    fn addition_subtraction_multiplication_and_division_are_left_associative() {
        assert_evaluates_to("20 - 5 - 3 =", 12.0);
        assert_evaluates_to("100 / 10 / 2 =", 5.0);
        assert_evaluates_to("2 * 3 / 4 =", 1.5);
    }

    #[test]
    fn evaluates_unary_negation_and_nested_parentheses() {
        assert_evaluates_to("-5 =", -5.0);
        assert_evaluates_to("--5 =", 5.0);
        assert_evaluates_to("-(2 + 3) =", -5.0);
        assert_evaluates_to("((2 + 3) * (4 - 1)) =", 15.0);
    }

    #[test]
    fn unary_negation_binds_before_exponentiation() {
        assert_evaluates_to("-2 ^ 2 =", 4.0);
        assert_evaluates_to("-(2 ^ 2) =", -4.0);
    }

    #[test]
    fn exponentiation_and_roots_are_right_associative() {
        assert_evaluates_to("2 ^ 3 ^ 2 =", 512.0);
        assert_evaluates_to("16 $ 2 ^ 2 =", 2.0);
        assert_evaluates_to("256 $ 2 $ 2 =", 256.0_f64.powf(1.0 / 2.0_f64.sqrt()));
    }

    #[test]
    fn evaluates_valid_roots() {
        assert_evaluates_to("27 $ 3 =", 3.0);
        assert_evaluates_to("16 $ 2 =", 4.0);
        assert_evaluates_to("-8 $ 3 =", -2.0);
    }

    #[test]
    fn supports_every_implicit_multiplication_pair() {
        assert_evaluates_to("2 3 =", 6.0);
        assert_evaluates_to("2(3 + 4) =", 14.0);
        assert_evaluates_to("(1 + 2)3 =", 9.0);
        assert_evaluates_to("(1 + 2)(3 + 4) =", 21.0);
    }

    #[test]
    fn rejects_division_by_zero_and_zero_index_roots() {
        assert_eq!(
            evaluate("1 / 0 ="),
            Err(CalcError::Math(MathError::DivisionByZero))
        );
        assert_eq!(
            evaluate("27 $ 0 ="),
            Err(CalcError::Math(MathError::DivisionByZero))
        );
    }

    #[test]
    fn rejects_invalid_negative_roots() {
        assert_eq!(
            evaluate("-16 $ 2 ="),
            Err(CalcError::Math(MathError::EvenRootOfNegative {
                base: -16.0,
                root: 2.0,
            }))
        );
        assert_eq!(
            evaluate("-8 $ 2.5 ="),
            Err(CalcError::Math(MathError::NegativeRoot {
                base: -8.0,
                root: 2.5,
            }))
        );
    }

    #[test]
    fn rejects_non_finite_power_and_root_results() {
        assert_eq!(
            evaluate("-4 ^ .5 ="),
            Err(CalcError::Math(MathError::InvalidExponentiation {
                base: -4.0,
                exponent: 0.5,
            }))
        );
        assert_eq!(
            evaluate("0 $ -1 ="),
            Err(CalcError::Math(MathError::InvalidRoot {
                base: 0.0,
                root: -1.0,
            }))
        );
    }

    #[test]
    fn detects_overflow_and_underflow() {
        let parser = MathExpressionParser::new(Vec::new());
        assert_eq!(
            parser.check_overflow(f64::INFINITY),
            Err(CalcError::Math(MathError::OverflowError))
        );
        assert_eq!(
            parser.check_overflow(f64::from_bits(1)),
            Err(CalcError::Math(MathError::UnderflowError))
        );
        assert_eq!(parser.check_overflow(0.0), Ok(0.0));
        assert_eq!(parser.check_overflow(f64::MAX), Ok(f64::MAX));
    }

    #[test]
    fn requires_a_final_terminator_and_rejects_trailing_tokens() {
        assert_eq!(
            evaluate("2 + 3"),
            Err(CalcError::Token(TokenError::UnexpectedEnd))
        );
        assert_eq!(
            evaluate("2 + 3 = 5"),
            Err(CalcError::Token(TokenError::UnexpectedToken(
                Token::Number(5.0)
            )))
        );
        assert_eq!(
            evaluate("2 + 3 =="),
            Err(CalcError::Token(TokenError::UnexpectedToken(Token::Equals)))
        );
    }

    #[test]
    fn rejects_missing_operands() {
        for input in ["=", "2 + =", "2 - =", "2 * =", "2 / =", "2 ^ =", "2 $ ="] {
            assert!(
                matches!(
                    evaluate(input),
                    Err(CalcError::Token(TokenError::InvalidExpression(_)))
                ),
                "expected invalid expression for {input:?}"
            );
        }
    }

    #[test]
    fn reports_unmatched_parentheses() {
        assert!(matches!(
            evaluate("(2 + 3 ="),
            Err(CalcError::Token(TokenError::UnmatchedParenthesis {
                found: ')',
                ..
            })) | Err(CalcError::Token(TokenError::UnmatchedParenthesis {
                found: '(',
                ..
            }))
        ));
        assert_eq!(
            evaluate("2 + 3) ="),
            Err(CalcError::Token(TokenError::UnexpectedToken(
                Token::RightParen
            )))
        );
    }

    #[test]
    fn error_messages_are_human_readable() {
        assert_eq!(
            MathError::DivisionByZero.to_string(),
            "Math error: division by zero"
        );
        assert_eq!(
            TokenError::InvalidOperator('%').to_string(),
            "Error: invalid operator '%'"
        );
        assert_eq!(
            CalcError::Token(TokenError::UnexpectedEnd).to_string(),
            "Parsing error: Error: expression ended unexpectedly"
        );
    }
}
