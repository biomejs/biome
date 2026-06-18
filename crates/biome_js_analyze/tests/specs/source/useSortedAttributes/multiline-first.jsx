// sortScope: "group", groups: [:RESERVED:, :IMPLICIT:, :CALLBACK:], multiline: "first"
// All multiline-value props come before all single-line props.
// Within each part, props are sorted by group order.

// Unsorted: single-line props before multiline onChange
<Hello disabled key="1" onClick={fn} name="John" onChange={(e) => {
  fn(e);
}} />;

// Unsorted: single-line props interleaved with multilines from different groups
<Hello disabled key="1" onClick={fn} ref={(el) => {
  domRef.current = el;
}} name="John" onChange={(e) => {
  fn(e);
}} />;

// Correctly ordered: multiline onChange first, then single-line props by group order
<Hello onChange={(e) => {
  fn(e);
}} key="1" disabled onClick={fn} name="John" />;

// Correctly ordered: multilines from RESERVED then CALLBACK first (group order),
// then single-line props
<Hello ref={(el) => {
  domRef.current = el;
}} onChange={(e) => {
  fn(e);
}} key="1" disabled onClick={fn} name="John" />;
