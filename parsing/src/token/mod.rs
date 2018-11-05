use pest::{error::Error as PestError, Parser};
use pest_derive::*;

#[derive(Parser)]
#[grammar = "token/lex.pest"]
struct Lex;

#[derive(Debug)]
pub enum Error<'input> {
    PestErr(PestError<Rule>),
    InvalidInteger,
    InvalidHex,
    InvalidBinary,
    InvalidFloat,
    UnknownKeyword,
    UnknownGrouping,
    UnknownSymbol(&'input str),

    UnknownErr,
}

#[derive(Debug, Clone)]
pub enum Token<'input> {
    Ident(&'input str),
    Integer(rug::Integer),
    Float(rug::Float),

    Operator(&'input str),

    Let,
    Prev,

    Comma,
    Semicolon,
    Equals,

    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
}

/// A span holding the start and end of a token
pub struct Span {
    pub start: u16,
    pub end: u16,
}

impl std::fmt::Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

impl<'a> From<pest::Span<'a>> for Span {
    fn from(pest_span: pest::Span) -> Self {
        Span {
            start: pest_span.start() as u16,
            end: pest_span.end() as u16,
        }
    }
}

#[derive(Debug)]
pub struct SpannedToken<'a>(pub Token<'a>, pub Span);

impl<'a> SpannedToken<'a> {
    pub fn new<S: Into<Span>>(token: Token<'a>, span: S) -> Self {
        SpannedToken(token, span.into())
    }
}

#[derive(Debug)]
pub struct SpannedError<'a>(pub Error<'a>, pub Option<Span>);

impl<'a> SpannedError<'a> {
    pub fn spanned<O, S: Into<Span>>(err: Error, span: S) -> Result<O, SpannedError> {
        Err(SpannedError(err, Some(span.into())))
    }

    pub fn spanless<O>(err: Error) -> Result<O, SpannedError> {
        Err(SpannedError(err, None))
    }
}

/// Tokenizes a string into a stream of tokens
pub fn tokenize(input: &str) -> Result<Vec<SpannedToken>, SpannedError> {
    let mut token_list = match Lex::parse(Rule::token_list, input) {
        Ok(token_list) => token_list,
        Err(e) => return SpannedError::spanless(Error::PestErr(e)),
    };
    let tokens = match token_list.next() {
        Some(tokens) => tokens.into_inner(),
        None => return SpannedError::spanless(Error::UnknownErr),
    };

    let mut output_tokens = Vec::new();

    for token in tokens {
        let tok = match token.as_rule() {
            Rule::ident => SpannedToken::new(Token::Ident(token.as_str()), token.as_span()),
            Rule::integer => {
                let stripped_int = token.as_str().replace('_', "");
                let int_token = match token.into_inner().next() {
                    Some(tok) => tok,
                    None => panic!("Probably not valid state"),
                };

                let int = match int_token.as_rule() {
                    Rule::decimal_int => match stripped_int.parse() {
                        Ok(i) => i,
                        Err(_) => {
                            return SpannedError::spanned(Error::InvalidInteger, int_token.as_span())
                        }
                    },
                    Rule::hex_int => {
                        let stripped_int = if let Some('x') = stripped_int.chars().skip(1).next() {
                            &stripped_int[2..]
                        } else {
                            &stripped_int[..stripped_int.len() - 1]
                        };
                        match rug::Integer::from_str_radix(stripped_int, 16) {
                            Ok(int) => int,
                            Err(_) => {
                                return SpannedError::spanned(Error::InvalidHex, int_token.as_span())
                            }
                        }
                    }
                    Rule::binary_int => {
                        let stripped_int = &stripped_int[2..];
                        match rug::Integer::from_str_radix(stripped_int, 2) {
                            Ok(int) => int,
                            Err(_) => {
                                return SpannedError::spanned(
                                    Error::InvalidBinary,
                                    int_token.as_span(),
                                )
                            }
                        }
                    }
                    _ => unreachable!(),
                };
                let mult = if detect_negative(&output_tokens) {
                    // Remove the negative sign from the generated tokens
                    output_tokens.remove(output_tokens.len() - 1);
                    -1
                } else {
                    1
                };
                let tok = Token::Integer(int * mult);
                SpannedToken::new(tok, int_token.as_span())
            }
            // Rule::integer => {
            //     let stripped_int = token.as_str().replace('_', "");
            //     let stripped_int = match stripped_int.chars().skip(1).next() {
            //         Some('x') | Some('b') => &stripped_int[2..],
            //         _ => &stripped_int,
            //     };
            //     let stripped_int = match stripped_int
            //         .chars()
            //         .skip(stripped_int.len().saturating_sub(1))
            //         .next()
            //     {
            //         Some('h') => &stripped_int[..stripped_int.len() - 1],
            //         _ => &stripped_int,
            //     };
            //     let int: rug::Integer = match stripped_int.parse() {
            //         Ok(i) => i,
            //         Err(_) => return SpannedError::spanned(Error::InvalidInteger, token.as_span()),
            //     };
            //     let mult = if detect_negative(&output_tokens) {
            //         // Remove the negative sign from the generated tokens
            //         output_tokens.remove(output_tokens.len() - 1);
            //         -1
            //     } else {
            //         1
            //     };
            //     let tok = Token::Integer(int * mult);
            //     SpannedToken::new(tok, token.as_span())
            // }
            Rule::float => {
                let stripped_float = token.as_str().replace('_', "");
                let incomplete_float = match rug::Float::parse(stripped_float) {
                    Ok(i) => i,
                    Err(_) => return SpannedError::spanned(Error::InvalidFloat, token.as_span()),
                };
                let float = rug::Float::with_val(53, incomplete_float);
                let mult = if detect_negative(&output_tokens) {
                    // Remove the negative sign from the generated tokens
                    output_tokens.remove(output_tokens.len() - 1);
                    -1.
                } else {
                    1.
                };
                let tok = Token::Float(float * mult);
                SpannedToken::new(tok, token.as_span())
            }
            Rule::operator => {
                // TODO: Validate operator?
                SpannedToken::new(Token::Operator(token.as_str()), token.as_span())
            }
            Rule::keyword => {
                let tok = match token.as_str() {
                    "let" => Token::Let,
                    "#" => Token::Prev,
                    _ => return SpannedError::spanned(Error::UnknownKeyword, token.as_span()),
                };
                SpannedToken::new(tok, token.as_span())
            }
            Rule::grouping_char => {
                let tok = match token.as_str() {
                    "(" => Token::LParen,
                    ")" => Token::RParen,
                    "[" => Token::LBracket,
                    "]" => Token::RBracket,
                    "{" => Token::LBrace,
                    "}" => Token::RBrace,
                    _ => return SpannedError::spanned(Error::UnknownGrouping, token.as_span()),
                };
                SpannedToken::new(tok, token.as_span())
            }
            Rule::seperator => {
                let tok = match token.as_str() {
                    " " | "\n" => continue,
                    "," => Token::Comma,
                    ";" => Token::Semicolon,
                    sym => return SpannedError::spanned(Error::UnknownSymbol(sym), token.as_span()),
                };
                SpannedToken::new(tok, token.as_span())
            }
            Rule::symbol => {
                let tok = match token.as_str() {
                    "=" => Token::Equals,
                    sym => return SpannedError::spanned(Error::UnknownSymbol(sym), token.as_span()),
                };
                SpannedToken::new(tok, token.as_span())
            }
            Rule::EOI => break,

            // Impossible (silent rules)
            Rule::token
            | Rule::ident_char
            | Rule::ident_start
            | Rule::digit
            | Rule::number
            | Rule::hex_digit
            | Rule::hex_int
            | Rule::binary_int
            | Rule::decimal_int
            | Rule::token_list => unreachable!(),
        };
        output_tokens.push(tok);
    }
    Ok(output_tokens)
}

fn detect_negative(prev_tokens: &[SpannedToken]) -> bool {
    let mut negative = false;
    // Is it previous token a negative sign?
    if let Some(i) = prev_tokens.len().checked_sub(1) {
        if let Some(SpannedToken(Token::Operator(op), _)) = prev_tokens.get(i) {
            if *op == "-" {
                negative = true;
                // Is the number before the negative sign a number?
                // If it is, we can assume that it is subtraction
                if let Some(i) = prev_tokens.len().checked_sub(2) {
                    if let Some(token) = prev_tokens.get(i) {
                        match token {
                            SpannedToken(Token::Integer(_), _)
                            | SpannedToken(Token::Float(_), _) => {
                                // It is subtaction, do not negate
                                negative = false;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
    negative
}
