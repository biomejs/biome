/* should not generate diagnostics */
const object = {...{read() { return super.value!; }}};
new Set([...iterable] as Iterable<number>);
const array = [...[a] as const];
