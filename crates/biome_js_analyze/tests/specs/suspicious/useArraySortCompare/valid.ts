/* should not generate diagnostics */
const array: any[] = [];
array.sort((a, b) => a - b);
array.sort((a, b) => a.localeCompare(b));
array.toSorted((a, b) => a - b);

const userDefinedType = {
	sort: () => { }
};
userDefinedType.sort();
