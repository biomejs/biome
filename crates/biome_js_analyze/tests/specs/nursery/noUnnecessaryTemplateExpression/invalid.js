/* should generate diagnostics */

// Single string literal interpolation
const a = `${'hello'}`;
const b = `${"world"}`;
const c = `${''}`;

// Multiple string literal interpolations (all can be combined)
const d = `${'hello'}${'world'}`;
const e = `${'hello'} ${'world'}`;

// String literal interpolation with surrounding text
const f = `prefix_${'suffix'}`;
const g = `${'prefix'}_suffix`;
const h = `before_${'middle'}_after`;

// Nested quotes - single in double and vice versa
const i = `${"it's fine"}`;
const j = `${'use "double" inside'}`;

// Both quote kinds present: diagnostic should remain, safe fix should not
const k = `${'"'}${"'"'}`;
