use std::borrow::Cow;

use rustyline::{
    completion::Completer, error::ReadlineError, highlight::Highlighter, hint::Hinter, Helper,
};

pub struct MathHelper;

impl Completer for MathHelper {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
    ) -> Result<(usize, Vec<Self::Candidate>), ReadlineError> {
        ().complete(line, pos)
    }
}

impl Hinter for MathHelper {
    fn hint(&self, _line: &str, _pos: usize) -> Option<String> {
        None
    }
}

/// Basic Pattern to match operators
fn op_formatter(ch: char) -> String {
    match ch {
        '*' | '/' | '+' | '-' | '%' | '!' => format!("\x1b[1m{}\x1b[0m", ch),
        '#' => format!("\x1b[31m{}\x1b[0m", ch),
        _ => ch.to_string(),
    }
}

impl Highlighter for MathHelper {
    // TODO: Use tokens/ast to highlight this better
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        let out = line.chars().map(op_formatter).collect();
        Cow::Owned(out)
    }
}

impl Helper for MathHelper {}
