/* should generate diagnostics */
array.reduce<Record<string, Data>>((result, entry) => ({...result, [entry.id]: entry.data}), {});
pairs.reduce((object: Record<string, number>, [key, value]: [string, number]) => ({...object, [key]: value}), {});
function convert(values: number[]) {
	return values.reduce((object, value) => ({...object, [value]: value}), {});
}
// The return type describes the object, not the pair returned after the fix.
pairs.reduce((object, [key, value]): Record<string, number> => ({...object, [key]: value}), {});
