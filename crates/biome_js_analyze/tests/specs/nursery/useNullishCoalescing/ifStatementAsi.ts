/* should generate diagnostics */
if ((obj as Holder).bar == null) (obj as Holder).bar = 'x'

type Holder = { bar?: string | null };
declare const obj: unknown;
declare function log(): () => unknown;
declare let target: string | null;

// A fix starting with `(` would continue the previous statement: no fix
log()
if ((obj as Holder).bar == null) (obj as Holder).bar = 'x'

// Parentheses around the target are removed, so the fix starts with the name
log()
if (target == null) (target) = 'x'

// Previous statement ends with `;`
log();
if ((obj as Holder).bar == null) (obj as Holder).bar = 'x'

// First statement of a block
{
	if ((obj as Holder).bar == null) (obj as Holder).bar = 'x'
}

// After the closing brace of a block: no fix
{
}
if ((obj as Holder).bar == null) (obj as Holder).bar = 'x'
