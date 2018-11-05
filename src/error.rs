use lalrpop_util::ParseError::UnrecognizedToken;

pub fn try_humanize(error: &crate::ast::TErrorRecovery) -> Option<String> {
    if let UnrecognizedToken { token: None, .. } = error.error {
        return Some("Unexpected end of input".to_owned());
    }
    if let UnrecognizedToken {
        token: Some(_),
        ref expected,
    } = error.error
    {
        if expected.contains(&"\"operator\"".to_owned()) {
            return Some("Possible missing operator".to_owned());
        }
    }
    None
}
