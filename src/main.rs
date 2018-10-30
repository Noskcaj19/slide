#[macro_use]
extern crate lalrpop_util;
extern crate liner;

use liner::Context;

mod ast;

lalrpop_mod!(pub calc);

fn error_to_range(err: &ast::ParseError) -> (usize, usize) {
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

fn print_errors(errs: &[ast::ParseError], len: usize) {
    for err in errs {
        let (start, end) = match error_to_range(err) {
            (0, 0) => (len, len),
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
}

fn print_parse_error<'input>(
    error: lalrpop_util::ParseError<usize, calc::Token<'input>, &'static str>,
    len: usize,
) {
    print_errors(
        &[lalrpop_util::ErrorRecovery {
            error,
            dropped_tokens: vec![],
        }],
        len,
    )
}

fn main() {
    let mut con = Context::new();

    loop {
        let res = con.read_line("> ", &mut |_| {}).unwrap();

        {
            let mut errors = Vec::new();
            let expr = calc::ExprsParser::new().parse(&mut errors, &res);
            let expr = match expr {
                Err(err) => {
                    print_parse_error(err, res.len());
                    continue;
                }
                Ok(expr) => expr,
            };

            println!("=> {:?}", expr);
            if errors.len() > 0 {
                print!("=> {}", res);
                print_errors(&errors, res.len());
                for l in format!("{:#?}", &errors).lines() {
                    println!("=# {}", l);
                }
            }
        }
    }
}
