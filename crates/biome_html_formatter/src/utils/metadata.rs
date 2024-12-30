/// HTML tags that have an "inline" layout by default.
///
/// In HTML, The inline layout treats the element as if it were a single line of text. This means that the element does not start on a new line, and only takes up as much width as necessary.
/// In contrast, block layout elements start on a new line and take up the full width of the parent element.
///
/// ### References
///  - Pretter uses: [html-ua-styles](https://github.com/prettier/html-ua-styles) to determined which tags are inline by default.
///  - HTML WHATWG spec: <https://html.spec.whatwg.org/multipage/rendering.html#the-css-user-agent-style-sheet-and-presentational-hints>
///  - <https://developer.mozilla.org/en-US/docs/Glossary/Inline-level_content>
///  - <https://developer.mozilla.org/en-US/docs/Glossary/Block-level_content>
pub const HTML_INLINE_TAGS: &[&str] = &[
    // TODO: this is incomplete. derive this from the HTML spec.
    "b", "i", "u", "span", "a", "strong", "em", "small", "big",
];
