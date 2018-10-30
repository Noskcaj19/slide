extern crate rug;
#[macro_use]
extern crate lalrpop_util;
extern crate liner;

use liner::Context;

mod ast;
mod eval;
mod number;

lalrpop_mod!(pub calc);

fn error_to_range(err: &ast::TErrorRecovery) -> (usize, usize) {
    use lalrpop_util::ParseError;
    match err.error {
        ParseError::ExtraToken {
            token: (start, _, end),
        } => (start, end),
        ParseError::InvalidToken { location } => (location, location),
        ParseError::UnrecognizedToken { ref token, .. } => {
            token.clone().map(|t| (t.0, t.2)).unwrap_or((0, 0))
        }
        ParseError::User { .. } => (0, 0),
    }
}

fn print_errors(errs: &[ast::TErrorRecovery], input: &str) {
    println!("=> {}", input);
    for err in errs {
        let (start, end) = match error_to_range(err) {
            (0, 0) => (input.len(), input.len()),
            l => l,
        };

        if end.saturating_sub(start) == 0 {
            println!("   {}^", " ".repeat(start.saturating_sub(1)))
        } else {
            println!(
                "   {}{}",
                " ".repeat(start.saturating_sub(1)),
                "~".repeat((end.saturating_sub(start)) + 1)
            )
        }
    }
    for l in format!("{:#?}", errs).lines() {
        println!("=# {}", l);
    }
}

fn print_parse_error<'input>(error: ast::TParseError, input: &str) {
    print_errors(
        &[lalrpop_util::ErrorRecovery {
            error,
            dropped_tokens: vec![],
        }],
        input,
    )
}

fn main() {
    let mut con = Context::new();
    let mut input;

    loop {
        input = con.read_line("> ", &mut |_| {}).unwrap();
        con.history.push(input.clone().into()).unwrap();

        let mut errors = Vec::new();

        let expr = calc::ExprParser::new().parse(&mut errors, &input);
        let expr = match expr {
            Err(err) => {
                print_parse_error(err, &input);
                continue;
            }
            Ok(expr) => expr,
        };

        if errors.len() > 0 {
            print_errors(&errors, &input);
        } else {
            println!("=> {}", eval::eval(*expr));
        }
    }
}
