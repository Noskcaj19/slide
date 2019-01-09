use rustyline::{error::ReadlineError, Config, Editor};

mod prompt_helper;

use slide::*;

struct SlideContext {
    editor_ctx: Editor<prompt_helper::MathHelper>,
    eval_ctx: eval::EvalContext,
}

impl SlideContext {
    fn new() -> SlideContext {
        let helper = prompt_helper::MathHelper;
        let config = Config::builder()
            .history_ignore_space(true)
            .auto_add_history(true)
            .build();
        let mut editor = Editor::with_config(config);
        editor.set_helper(Some(helper));
        SlideContext {
            editor_ctx: editor,
            eval_ctx: eval::EvalContext::new(),
        }
    }

    pub fn eval_line(&mut self, input: &str) {
        let tokens = match token::tokenize(input) {
            Ok(tokens) => tokens,
            Err(e) => {
                self.print_lex_error(e);
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
        let mut dump = true;
        for err in errs {
            let (start, end) = match error_to_range(err) {
                (0, 0) => (input.len(), input.len()),
                l => l,
            };

            if end.saturating_sub(start) <= 1 {
                println!("   {}^", " ".repeat(start))
            } else {
                println!(
                    "   {}{}",
                    " ".repeat(start),
                    "~".repeat(end.saturating_sub(start))
                )
            }
            if let Some(human_err) = error::try_humanize(err) {
                println!("=# {}", human_err);
                dump = false;
            }
        }
        if dump {
            for err in errs {
                println!("=# =====");
                for l in format!("{:#?}", err).lines() {
                    println!("=# {}", l);
                }
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

    fn print_lex_error(&self, err: parsing::token::SpannedError<'_>) {
        match err.0 {
            parsing::token::Error::PestErr(pest_err) => {
                let (start, _) = parsing::token::span_from_loc(pest_err.location);
                println!("   {}~", " ".repeat(start));
                println!("=# Invalid token");
            }
            _ => println!("{:#?}", err),
        }
    }
}

fn main() {
    let mut slide_ctx = SlideContext::new();

    loop {
        let input = match slide_ctx.editor_ctx.readline("<< ") {
            Ok(line) => line,
            Err(ReadlineError::Interrupted) => {
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
