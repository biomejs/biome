// sortScope: "group", groups: [:RESERVED:, :IMPLICIT:, :CALLBACK:]
// Spread props act as fences: each batch between spreads is sorted independently.

// Unsorted batch before a spread
<Hello onChange={fn} disabled key="1" {...this.props} />;

// Unsorted batch after a spread
<Hello {...this.props} onChange={fn} disabled key="1" />;

// Both batches unsorted
<Hello onChange={fn} disabled key="1" {...this.props} onChange={fn} disabled key="2" />;

// First batch sorted, second unsorted
<Hello key="1" disabled onChange={fn} {...this.props} onChange={fn} disabled key="2" />;

// Correctly sorted: all batches already in group order — no diagnostic
<Hello key="1" disabled onChange={fn} {...this.props} key="2" disabled onChange={fn} />;

// Multiple spreads: each segment sorted independently
<Hello onChange={fn} disabled {...props1} lastName="Smith" firstName="John" {...props2} onChange={fn} key="1" />;
