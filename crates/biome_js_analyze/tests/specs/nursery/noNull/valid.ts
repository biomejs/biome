/* should not generate diagnostics */
let a: string | null;
type Nullable<T> = T | null;
function check(value: unknown): value is null {
	return value === (null as unknown);
}
if (foo === null!) {}
Object.create(null as object | null);
const ref = useRef<HTMLDivElement>(null);
const reactRef = React.useRef<HTMLDivElement>(null);
