extern crate lalrpop_util;
extern crate liner;
extern crate slide;
extern crate termion;

use liner::{Context as LineContext, Event, EventKind};
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

struct SlideContext {
    line_ctx: LineContext,
    eval_ctx: eval::EvalContext,
}

impl SlideContext {
    fn new() -> SlideContext {
        SlideContext {
            line_ctx: LineContext::new(),
            eval_ctx: eval::EvalContext::new(),
        }
    }

    pub fn eval_line(&mut self, input: &str) {
        self.line_ctx.history.push(input.clone().into()).unwrap();
        let mut errors = Vec::new();

        let expr = match calc::ExprParser::new().parse(&mut errors, input) {
            Err(err) => {
                self.print_parse_error(err, &input);
                return;
            }
            Ok(expr) => expr,
        };

        if errors.len() > 0 {
            self.print_errors(&errors, &input);
        } else {
            println!("=> {}", self.eval(*expr));
        }
    }

    fn eval(&mut self, expr: ast::Expr) -> &ast::Number {
        self.eval_ctx.eval(expr)
    }

    fn print_errors(&self, errs: &[ast::TErrorRecovery], input: &str) {
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

    fn print_parse_error(&self, error: ast::TParseError, input: &str) {
        self.print_errors(
            &[lalrpop_util::ErrorRecovery {
                error,
                dropped_tokens: vec![],
            }],
            input,
        )
    }

    // TODO: Currently a hack
    fn handle_event<W: std::io::Write>(&mut self, event: Event<W>) {
        if event.editor.cursor() != 0 {
            return;
        }
        match event.kind {
            EventKind::BeforeKey(Key::Char(ch)) => match ch {
                '>' | '<' | '-' | '+' | '*' | '(' | '/' => {
                    event.editor.insert_str_after_cursor("#").unwrap();
                }
                _ => {}
            },
            _ => {}
        };
    }
}

fn main() {
    let mut line_ctx = LineContext::new();
    let mut slide_ctx = SlideContext::new();

    loop {
        let input = match line_ctx.read_line("> ", &mut |e| slide_ctx.handle_event(e)) {
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
        slide_ctx.eval_line(&input)
    }
}
