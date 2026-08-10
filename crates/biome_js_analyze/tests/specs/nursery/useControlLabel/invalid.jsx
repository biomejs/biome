/* should generate diagnostics */

const a = <button />;

const b = <button></button>;

const c = <menuitem />;

// `aria-hidden="false"` is still exposed — a label is still required.
const d = <button aria-hidden="false" />;

// An empty labeling attribute is not a real label.
const e = <button aria-label="" />;

// A hidden child does not supply an accessible label.
const f = <button><span aria-hidden="true">Delete</span></button>;

// An empty template-literal label is not a real label either.
const g = <button aria-label={``} />;
