/* should generate diagnostics */
let a: string | null = null;
let b = null as unknown;
const c = <unknown>null;
delete (null as unknown);
delete (null satisfies unknown);
delete (<unknown>null);
delete null!;
using d = null;
for (using e = null; ; ) { break; }
enum Values {
	Empty = null,
}
