pub struct KindsSrc<'a> {
    /// Special characters of the language. Usually these are parenthesis, dots, commas, etc.
    pub punct: &'a [(&'a str, &'a str)],
    /// Reserved keywords of the language
    pub keywords: &'a [&'a str],
    /// Literals are special nodes that holds some **values** inside the language, for example: strings, numbers, etc.
    pub literals: &'a [&'a str],
    /// Whitespaces, comments, identifiers, etc.
    pub tokens: &'a [&'a str],
    /// Nodes of the CST. Usually you want to map these names from the `.ungram` file. For example:
    ///
    /// HtmlAttribute -> HTML_ATTRIBUTE
    /// HtmlBogus -> HTML_BOGUS
    pub nodes: &'a [&'a str],
}
