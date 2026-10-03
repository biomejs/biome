/* should not generate diagnostics */

// Variable expressions (type unknown without type info)
const a = `${someVariable}`;
const b = `${someVariable} suffix`;
const c = `prefix ${someVariable}`;
const d = `${a} and ${b}`;

// Tagged templates are never flagged
const e = html`${'foo'}`;
const f = css`${'color: red'}`;

// Number and boolean literals (they perform a type conversion, so they're potentially intentional)
const g = `${42}`;
const h = `${true}`;
const i = `${null}`;

// Template with a literal newline in the text part
const j = `line one
${'line two'}`;

// Template with no interpolations (handled by noUnusedTemplateLiteral)
const k = `just a string`;

// Already a plain string
const l = 'hello world';