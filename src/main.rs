use liner::{Context as LineContext, Event, EventKind};
use termion::event::Key;

use slide::{ast, eval, token};

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
        self.line_ctx.history.push(input.into()).unwrap();

        let tokens = match token::tokenize(input) {
            Ok(tokens) => tokens,
            Err(e) => {
                println!("Tokenizer error: {:?}", e);
                return;
            }
        };

        // Translate the tokens into a form lalrpop likes
        let mut lalr_tokens = Vec::new();
        for token in tokens {
            lalr_tokens.push(Ok((token.1.start as usize, token.0, token.1.end as usize)))
        }

        let mut errors = Vec::new();
        let nodes = match ast::parse(&mut errors, lalr_tokens) {
            Err(err) => {
                self.print_parse_error(err, &input);
                return;
            }
            Ok(n) => n,
        };

        if errors.is_empty() {
            println!("=> {}", self.eval(nodes[0].clone()));
        } else {
            self.print_errors(&errors, &input);
        }
    }

    fn eval(&mut self, node: ast::Node) -> &ast::Number {
        self.eval_ctx.eval(node)
    }

    fn print_errors(&self, errs: &[ast::TErrorRecovery], input: &str) {
        println!("=> {}", input);
        for err in errs {
            let (start, end) = match error_to_range(err) {
                (0, 0) => (input.len(), input.len()),
                l => l,
            };

            if end.saturating_sub(start) == 0 {
                println!("   {}^", " ".repeat(start))
            } else {
                println!(
                    "   {}{}",
                    " ".repeat(start),
                    "~".repeat(end.saturating_sub(start))
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
        if let EventKind::BeforeKey(Key::Char(ch)) = event.kind {
            match ch {
                '>' | '<' | '-' | '+' | '*' | '(' | '/' => {
                    event.editor.insert_str_after_cursor("#").unwrap();
                }
                _ => {}
            }
        };
    }
}

fn main() {
    let mut line_ctx = LineContext::new();
    let mut slide_ctx = SlideContext::new();

    loop {
        let input = match line_ctx.read_line("<< ", &mut |e| slide_ctx.handle_event(e)) {
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
        // TODO: Fix this
        line_ctx.history.push(input.clone().into()).unwrap();
        slide_ctx.eval_line(&input)
    }
}

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
