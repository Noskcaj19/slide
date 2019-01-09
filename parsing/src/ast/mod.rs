mod number;

pub use self::number::Number;

use lalrpop_util::lalrpop_mod;

/// Custom parsing errors
#[derive(Debug, PartialEq)]
pub enum Error {}

pub type TErrorRecovery<'input> =
    lalrpop_util::ErrorRecovery<usize, crate::token::Token<'input>, Error>;
pub type TParseError<'input> = lalrpop_util::ParseError<usize, crate::token::Token<'input>, Error>;

lalrpop_mod!(
    #[allow(clippy::all)]
    grammar
);

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Number(Number),

    String(String),
    Ident(String),

    Infix {
        lhs: Box<Node>,
        op: String,
        rhs: Box<Node>,
    },
    FunctionDef {
        name: String,
        params: Vec<String>,
        body: Vec<Node>,
    },
    FunctionCall {
        name: String,
        args: Vec<Node>,
    },

    Let(String, Box<Node>),
    Prev,
    Error,
}

pub fn parse<'input, 'err>(
    errors: &'err mut Vec<TErrorRecovery<'input>>,
    tokens: Vec<Result<(usize, crate::token::Token<'input>, usize), Error>>,
) -> Result<Vec<Node>, TParseError<'input>> {
    let ast = grammar::NodesParser::new().parse(errors, tokens.into_iter())?;

    // TODO: Reparse

    Ok(ast)
}

pub fn parse_single<'input, 'err>(
    errors: &'err mut Vec<TErrorRecovery<'input>>,
    tokens: Vec<Result<(usize, crate::token::Token<'input>, usize), Error>>,
) -> Result<Node, TParseError<'input>> {
    let ast = grammar::NodeParser::new().parse(errors, tokens.into_iter())?;

    // TODO: Reparse

    Ok(ast)
}
