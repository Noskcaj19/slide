use lalrpop_util::ParseError::UnrecognizedToken;

pub fn try_humanize(error: &crate::ast::TErrorRecovery) -> Option<String> {
    if let UnrecognizedToken { token: None, .. } = error.error {
        return Some("Unexpected end of input".to_owned());
    }
    None
}
