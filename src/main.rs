extern crate lalrpop_util;
extern crate liner;
extern crate slide;
extern crate termion;

use liner::{Context, Event, EventKind};
use termion::event::Key;

use slide::{ast, calc, eval};

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
    for err in errs {
        println!("=# =====");
        for l in format!("{:#?}", err).lines() {
            println!("=# {}", l);
        }
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

// TODO: Currently a hack
fn handle_event<W: std::io::Write>(event: Event<W>) {
    if event.editor.cursor() != 0 {
        return;
    }
    let result = {
        let ctx = event.editor.context();
        let last_expr = match ctx.history.buffers.get(ctx.history.len().saturating_sub(1)) {
            Some(l) => l,
            None => return,
        };
        let input = &last_expr.lines()[0];
        let mut errors = Vec::new();
        let expr = match calc::ExprParser::new().parse(&mut errors, input) {
            Ok(e) => e,
            Err(_) => return,
        };
        if errors.len() != 0 {
            return;
        }
        eval::eval(*expr)
    };
    match event.kind {
        EventKind::BeforeKey(Key::Char(ch)) => match ch {
            '>' | '<' | '-' | '+' | '*' | '(' | '/' => {
                let _ = event.editor.insert_str_after_cursor(&format!("{}", result));
            }
            _ => {}
        },
        _ => {}
    };
}

fn main() {
    let mut con = Context::new();
    let mut input;

    loop {
        input = match con.read_line("> ", &mut handle_event) {
            Ok(line) => line,
            Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => {
                // ctrl-c
                // TODO: Figure out how to access the inner value and check it
                break;
            }
            Err(e) => {
                for l in format!("{:#?}", e).lines() {
                    println!("=# {}", l);
                }
                continue;
            }
        };
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
