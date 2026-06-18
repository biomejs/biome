// sortScope: "group", groups: [:REST:, :CALLBACK:], multiline: "first"
// Normally-formatted JSX (one prop per line). Only `data` has a multiline
// VALUE; the other props are single-line even though each sits on its own
// source line. `multiline: "first"` must pull only `data` to the front, not
// treat every own-line prop as multiline.

// Unsorted: multiline-value `data` should move before the single-line props
<Hello
  name="John"
  data={{
    a: 1,
  }}
  onClick={fn}
/>;

// Correctly ordered: multiline `data` first, then REST (name), then CALLBACK (onClick)
<Hello
  data={{
    a: 1,
  }}
  name="John"
  onClick={fn}
/>;
