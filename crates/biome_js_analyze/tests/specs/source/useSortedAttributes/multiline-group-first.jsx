// sortScope: "group", groups: [:RESERVED:, :IMPLICIT:, :CALLBACK:], multiline: "groupFirst"
// Within each group, multiline-value props are sub-grouped before single-line-value props.

// Unsorted: single-line onClick before multiline onChange in CALLBACK group,
// and key/disabled out of group order
<Hello onClick={fn} onChange={(event) => {
  handleChange(event);
}} key="1" disabled name="John" />;

// Unsorted: single-line key before multiline ref in RESERVED group
<Hello key="1" ref={(el) => {
  domRef.current = el;
}} onClick={fn} name="John" />;

// Both callback props have multiline values: sorted alphabetically (onChange < onClick)
<Hello onClick={(e) => {
  fn1(e);
}} onChange={(e) => {
  fn2(e);
}} key="1" name="John" />;

// Correctly ordered: RESERVED [multiline ref, single-line key],
// CALLBACK [multiline onChange, single-line onClick], REST [name]
<Hello ref={(el) => {
  domRef.current = el;
}} key="1" onChange={(event) => {
  handleChange(event);
}} onClick={fn} name="John" />;

// Correctly ordered: no multiline-value props — groupFirst has no effect
<Hello key="1" disabled onClick={fn} name="John" />;
