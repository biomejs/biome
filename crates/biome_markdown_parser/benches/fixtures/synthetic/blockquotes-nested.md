> This is a simple single-level blockquote.
> It spans multiple lines and contains regular text.
> The parser should handle this without difficulty.

> First level of nesting.
>
> > Second level of nesting.
> > This continues on the second level.
> >
> > > Third level of nesting.
> > > Still on the third level.
> > >
> > > > Fourth level.
> > > > Going deeper.
> > > >
> > > > > Fifth level of nesting.
> > > > > This is getting quite deep.
> > > > >
> > > > > > Sixth level, the deepest in this block.
> > > > > > Testing the parser's depth handling.

> Back to level one after the deep nesting.

> Blockquote with lazy continuation.
> This line has the marker.
This line does not have the marker but continues the quote.
And this one is also a lazy continuation.
> Back to having the marker again.
> Another properly marked line.
This is lazy again.

> > Nested quote with lazy continuation.
> > First line is marked.
> > Second line is marked.
Lazy continuation of the inner quote.
Another lazy line.
> > Back to marked lines.

> Blockquote containing a list:
>
> - Item one in the quote
> - Item two in the quote
>   - Nested item in the quote
>   - Another nested item
> - Item three
>
> And a paragraph after the list.

> Blockquote with a code block:
>
> ```javascript
> function greet(name) {
>     return `Hello, ${name}!`;
> }
> ```
>
> And text after the code block.

> Blockquote with **bold text**, *italic text*, and `inline code`.
> Also contains a [link](https://example.com) and an ![image](photo.jpg).

> > > Triple nested with emphasis: ***bold and italic*** inside.
> > > And `code spans` at multiple levels.
> >
> > Back to double nesting.
> > With some *emphasized* words.
>
> Back to single nesting.
> Final line at this level.

> First quote block.
> Has multiple lines.
> Ends here.

> Second quote block immediately after.
> These are separate blockquotes.

> Quote with multiple paragraphs.
>
> This is the second paragraph inside the same blockquote.
> It continues for a couple of lines.
>
> And a third paragraph for good measure.
> With continuation text.

> > Nested quote with multiple paragraphs.
> >
> > Second paragraph in nested quote.
> >
> > > Deeper still with its own paragraphs.
> > >
> > > Second paragraph at the deepest level.

> Mixed content blockquote:
>
> 1. Ordered list item one
> 2. Ordered list item two
>    - Sub-bullet A
>    - Sub-bullet B
> 3. Ordered list item three
>
> > Nested blockquote after the list.
> > With its own content.
>
> Final paragraph in the outer quote.

> > > > > Deep quote one-liner.

> Long blockquote to add size. The parser needs varied content at
> different nesting levels to properly exercise all code paths.
> This includes handling of continuation lines, blank lines within
> quotes, and transitions between nesting levels. Each of these
> scenarios triggers different logic in the block structure phase
> of the parser, making them valuable for benchmarking purposes.
