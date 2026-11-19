/* should not generate diagnostics */
declare function returnAny(): any;

function someFn<MyGeneric>(): void;
function someFn<MyGeneric>(): void {
	const value: MyGeneric = returnAny();
	console.log(value);
}

someFn();
