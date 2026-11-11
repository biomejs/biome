/* should generate diagnostics */
(array as any[]).length === 0 || (array as any[]).every(Boolean);
(array as any[]).length > 0 && (array as any[]).some(Boolean);
array!.length === 0 || array!.every(Boolean);
array!.length > 0 && array!.some(Boolean);
(array satisfies number[]).length > 0 && (array satisfies number[]).some((value) => value > 0);
(<number[]>array).every((value) => value > 0) || (<number[]>array).length === 0;
function f(array: Int8Array) {
	return array.length === 0 || array.every(Boolean);
}
if ((array as any[]).length > 0) { for (const element of (array as any[])) { use(element); } }
if ((<any[]>array).length > 0) { for (const element of (<any[]>array)) { use(element); } }
if (array!.length !== 0) { for (const element of array!) { use(element); } }
if ((array satisfies any[]).length > 0) { for (const element of (array satisfies any[])) { use(element); } }
