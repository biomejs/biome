// sortScope: "group" with default groups
// Default group order: [:IMPLICIT:, :RESERVED:, :DOM_RESERVED:, :CALLBACK:, :MULTILINE:, ...rest]

// Correctly ordered: implicit first, then rest alphabetically
<Hello disabled name="John" onChange={fn} />;

// Unsorted: callback before implicit
<Hello onChange={fn} disabled name="John" />;

// Unsorted: key not first (reserved group)
<Hello name="John" key="1" />;

// Unsorted: children before regular props but after key/ref
<Hello name="John" key="1" children={<Foo />} />;

// Correctly ordered: key, ref, then rest
<Hello key="1" ref={myRef} name="John" />;

// Unsorted: callback before implicit and rest out of order
<Hello onFocus={fn} onClick={fn} active name="John" />;
