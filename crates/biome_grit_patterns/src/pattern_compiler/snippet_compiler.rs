use grit_util::{ByteRange, Language};
use std::borrow::Cow;

/// Takes a snippet with metavariables and returns a list of ranges and the
/// corresponding metavariables.
///
/// The ranges are in descending order.
pub fn split_snippet<'a>(snippet: &'a str, lang: &impl Language) -> Vec<(ByteRange, Cow<'a, str>)> {
    let mut ranges_and_metavars: Vec<(ByteRange, Cow<str>)> = Vec::new();

    let variable_regex = lang.metavariable_regex();
    let curly_var_regex = lang.metavariable_bracket_regex();

    for m in variable_regex.find_iter(snippet) {
        ranges_and_metavars.push(((m.start()..m.end()).into(), m.as_str().into()));
    }
    for m in curly_var_regex.find_iter(snippet) {
        let mut metavar: Cow<str> = m.as_str()[2..m.as_str().len() - 1].into();
        metavar.to_mut().insert(0, '$');
        ranges_and_metavars.push(((m.start()..m.end()).into(), metavar));
    }

    // Sort ranges in descending order
    ranges_and_metavars.sort_by(|a, b| b.0.start.cmp(&a.0.start));

    ranges_and_metavars
}
