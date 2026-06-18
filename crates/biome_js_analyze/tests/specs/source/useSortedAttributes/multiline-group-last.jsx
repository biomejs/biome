// sortScope: "group", groups: [:RESERVED:, :IMPLICIT:, :CALLBACK:], multiline: "groupLast"
// Within each group, single-line-value props come before multiline-value props.

// Unsorted: multiline ref before single-line key in RESERVED group,
// multiline onChange before single-line onClick in CALLBACK group
<Hello ref={(el) => {
  domRef.current = el;
}} key="1" onChange={(e) => {
  handleChange(e);
}} onClick={fn} name="John" />;

// Unsorted: group order wrong (callback before reserved) and multiline order wrong
<Hello onChange={(e) => {
  handleChange(e);
}} onClick={fn} ref={(el) => {
  domRef.current = el;
}} key="1" name="John" />;

// Correctly ordered: RESERVED [single-line key, multiline ref],
// CALLBACK [single-line onClick, multiline onChange], REST [name]
<Hello key="1" ref={(el) => {
  domRef.current = el;
}} onClick={fn} onChange={(e) => {
  handleChange(e);
}} name="John" />;

// Correctly ordered: no multiline-value props — groupLast has no effect
<Hello key="1" disabled onClick={fn} name="John" />;
