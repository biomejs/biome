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

// Truthiness check: a fix starting with `(` would continue the previous statement: no fix
type ObjectHolder = { bar?: Date | null };
declare const objectHolder: unknown;
log()
if (!(objectHolder as ObjectHolder).bar) (objectHolder as ObjectHolder).bar = new Date()

// Previous statement ends with `;`
log();
if (!(objectHolder as ObjectHolder).bar) (objectHolder as ObjectHolder).bar = new Date()

// Truthiness ternary: a fix starting with `(` would continue the previous statement: no fix
declare const left: { c: Date | null };
log()
!(left).c ? new Date() : left.c

// Previous statement ends with `;`
log();
!(left).c ? new Date() : left.c
