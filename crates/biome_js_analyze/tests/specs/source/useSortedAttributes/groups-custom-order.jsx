// Custom groups order: [:CALLBACK:, :RESERVED:, :IMPLICIT:]
// Callbacks first, then reserved (key/ref), then implicit (shorthand), then rest

// Unsorted: implicit before callback
<Hello disabled key="1" onChange={fn} name="John" />;

// Unsorted: reserved (key) after callback but implicit in wrong position
<Hello name="John" onChange={fn} disabled key="1" />;

// Correctly ordered: callback, reserved, implicit, rest
<Hello onChange={fn} key="1" disabled name="John" />;
