/* should generate diagnostics */

const a = <button />;

const b = <button></button>;

const c = <area />;

const d = <menuitem />;

// `aria-hidden="false"` is still exposed — a label is still required.
const e = <button aria-hidden="false" />;

// An empty labeling attribute is not a real label.
const f = <button aria-label="" />;
