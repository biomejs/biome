// sortScope: "group", groups: [:RESERVED:, :IMPLICIT:, :CALLBACK:], multiline: "group" (default)
// Spread fences apply independently: each segment is sorted by group order.

// Unsorted segment before spread: CALLBACK before RESERVED and IMPLICIT
<Hello onChange={(e) => {
  fn(e);
}} disabled key="1" {...rest} name="John" />;

// Both segments unsorted
<Hello onChange={(e) => {
  fn(e);
}} disabled {...rest} onBlur={(e) => {
  fn(e);
}} key="1" />;

// Correctly sorted: RESERVED, IMPLICIT, CALLBACK in each segment
<Hello key="1" disabled onChange={(e) => {
  fn(e);
}} {...rest} name="John" />;
