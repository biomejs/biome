// sortScope: "group", groups: [:RESERVED:, :IMPLICIT:, :CALLBACK:], multiline: "last"
// Single-line props come before multiline-value props (sorted by groups within each part).
// Note: is_multiline_prop checks text_with_trivia(), so a prop on its own line in a
// multiline JSX element is also "multiline". Use inline JSX to isolate multiline values.

// Unsorted: multiline-value style before single-line name
<Hello style={{
  color: "red",
}} name="John" />;

// Unsorted: multiline-value callback before single-line callback and REST prop
<Hello onClick={
  () => doSomething()
} name="John" onChange={fn} />;

// Correctly ordered: single-line REST first, then multiline-value REST
<Hello name="John" style={{
  color: "red",
}} />;

// Correctly ordered: single-line CALLBACK then REST, then multiline-value CALLBACK
<Hello onChange={fn} name="John" onClick={
  () => doSomething()
} />;
