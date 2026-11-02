/* should generate diagnostics */
function getSize(set: Set<string>) {
	return Array.from(set).length;
}

function getReadonlySize(set: ReadonlySet<string>) {
	return [...set].length;
}

function getAssertedSize(set: unknown) {
	return [...(set as Set<string>)].length;
}

function getAssertedFromSize(set: unknown) {
	return Array.from(set as Set<string>).length;
}

function getSatisfiedSize(set: Set<string>) {
	return Array.from(set satisfies ReadonlySet<string>).length;
}

function getNonNullSize(set?: Set<string>) {
	return [...set!].length;
}

let annotated: Set<number> = new Set();
[...annotated].length;

const constant = new Set<number>() as Set<number>;
[...constant].length;

declare function getSet(): Set<number>;
new [...getSet() as Set<number>].length.constructor("42");

const set = new Set();
use()
Array.from(set as Set<number>).length;

function getTypeAssertionSize(set: unknown) {
	return [...<Set<string>>set].length;
}
