/* should not generate diagnostics */

// Text content supplies the label.
const a = <button>Submit</button>;
const b = (
	<button>
		<span>Delete</span>
	</button>
);

// Labeling attributes supply the name.
const c = <button aria-label="Close" />;
const d = <button title="Close" />;
const e = <area alt="Region" href="#" />;

// Hidden from assistive technology — no label required.
const f = <button aria-hidden />;

// Not a control element.
const g = <div />;

// A custom component is not a native control.
const h = <CustomButton />;
