[example]: https://example.com "Example Title"

Full reference: [click here][example]

Collapsed reference: [example][]

Shortcut reference: [example]

[foo]: https://foo.com

Image full: ![alt text][foo]

Image collapsed: ![foo][]

Image shortcut: ![foo]

Multiple words in text: [click here for more info][example]

Empty label (collapsed): [test][]

[test]: https://test.com

Shortcut that looks like text: [undefined]

Mixed with inline: [inline](https://inline.com) and [ref][example]

Nested in paragraph: This is a paragraph with [a reference][foo] in the middle.

[Case Label]: https://case.example

Case-insensitive: [case label]

Whitespace normalized: [case   label]

[label\]]: https://escaped.example

Escaped bracket in label: [text][label\]]
