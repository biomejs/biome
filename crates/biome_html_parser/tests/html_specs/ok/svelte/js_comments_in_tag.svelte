<button
  // single-line comment
  onclick={doTheThing}
>click me</button>

<div
  /* block comment */
  class="foo"
>text</div>

<input
  /* multi-line
     block comment */
  type="text"
/>

<span
  // comment before attr
  data-test="value"
  /* comment between attrs */
  id="my-span"
>content</span>


<button
  // single-line comment
  onclick={doTheThing}
  // comment after attr
  {type}
  // comment after shorthand
>click me</button>

<div /* block comment */ class="foo" /* block comment */>text</div>
