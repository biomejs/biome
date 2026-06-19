// Custom groups with an explicit :REST: placement: [:RESERVED:, :REST:, :CALLBACK:]
// :REST: catches everything not explicitly listed (including implicit/dom-reserved
// props, since :IMPLICIT: and :DOM_RESERVED: aren't configured here) and sorts
// them like a regular group, instead of leaving them unsorted at the end.

// Unsorted: reserved not first, rest props out of order
<Hello zebra="z" key="1" apple="a" onClick={fn} disabled />;

// Correctly ordered: reserved, then rest (sorted, including "disabled"), then callback
<Hello key="1" apple="a" disabled zebra="z" onClick={fn} />;
