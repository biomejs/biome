pub const LANGUAGE_PREFIXES: [&str; 7] = ["js_", "ts_", "jsx_", "tsx_", "css_", "json_", "html_"];

pub struct KindsSrc<'a> {
    pub punct: &'a [(&'a str, &'a str)],
    pub keywords: &'a [&'a str],
    pub literals: &'a [&'a str],
    pub tokens: &'a [&'a str],
    pub nodes: &'a [&'a str],
}
