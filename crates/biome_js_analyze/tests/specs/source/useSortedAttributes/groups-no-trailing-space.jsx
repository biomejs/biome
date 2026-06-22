// sortScope: "group", groups: [:CALLBACK:, :RESERVED:]
// Regression: the source-final prop carries no trailing whitespace (it sits
// right before `/>` or `>`). When a group moves it off the end, the result must
// still keep a separating space, otherwise the props mash into invalid JSX
// (e.g. `onClick={fn}key="1"`).

// Self-closing, no space before `/>`: reserved key moved after callback
<Hello key="1" onClick={fn}/>;

// Opening element, no space before `>`
<Hello key="1" onClick={fn}>x</Hello>;

// Already in group order (callback, then reserved): no diagnostic
<Hello onClick={fn} key="1"/>;
