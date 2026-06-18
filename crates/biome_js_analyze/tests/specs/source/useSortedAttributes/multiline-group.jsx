// sortScope: "group", groups: [:RESERVED:, :IMPLICIT:, :CALLBACK:], multiline: "group"
// Multiline-value props sort alphabetically within their group — no special positioning.

// Unsorted within CALLBACK: onClick before onChange (onChange has multiline value but still sorts first)
<Hello key="1" disabled onClick={fn} onChange={(e) => {
  fn(e);
}} />;

// Unsorted group order and within group
<Hello onClick={fn} onChange={(e) => {
  fn(e);
}} disabled key="1" />;

// Correctly ordered: RESERVED, IMPLICIT, CALLBACK — onChange sorts before onClick alphabetically
<Hello key="1" disabled onChange={(e) => {
  fn(e);
}} onClick={fn} />;
