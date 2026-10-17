/* should generate diagnostics */
array.sort((a: number, b: number): boolean => a > b);
array.sort((a: number, b: number): boolean => compare(a, b));
array.sort(function (a: number, b: number): boolean {
	return compare(a, b);
});
array.sort((a: number, b: number): boolean => {
	if (a > b) {
		return true;
	}

	return false;
});
array.sort((a: number, b: number): a is number => compare(a, b));
array.sort((a: number, b: number): true | false => compare(a, b));
array.sort((a: number, b: number): Flag => compare(a, b));
array.sort((a: number, b: number) => (a > b) as boolean);
array.sort((a: number, b: number) => <boolean>compare(a, b));
array.sort((a: number, b: number) => compare(a, b) satisfies boolean);
array.sort((a: number, b: number) => (a > b)!);
array.sort((a: unknown, b: unknown) => (a as number) > (b as number));
array.sort((a, b) => (a as number) > b);
array.sort(<T extends { score: number }>(a: T, b: T) => a.score > b.score);
array.sort((a?, b?) => a > b);
array.sort((Boolean as (value: unknown) => boolean));
array.sort(((a, b) => a > b) as Comparator);
array.sort(compare as Comparator);
array.sort(compare as (a: number, b: number) => boolean);
array.sort(compare as CallableComparator);
array.sort(compare as { (a: number, b: number): boolean });
array.sort((compare as (a: number, b: number) => boolean)!);
array.sort(Boolean!);
array.sort(annotatedComparator);
array.sort(declaredComparator);
array.sort((a, b) => flag);
array.sort(<T,>(a: T, b: T) => a > b);
array.sort((a: number, b: number) => annotatedFlag);

function withParameters(parameterComparator: (a: number, b: number) => boolean, typedFlag: boolean) {
	array.sort(parameterComparator);
	array.sort((a, b) => typedFlag);
}

function typed(array: Int8Array) {
	array.sort((a, b) => a > b);
}

type Flag = boolean;
type Comparator = (a: number, b: number) => boolean;
interface CallableComparator {
	(a: number, b: number): boolean;
}
const annotatedComparator: (a: number, b: number) => boolean = (a, b) => compareUnknown(a, b);
declare function declaredComparator(a: number, b: number): boolean;
const flag = true;
let annotatedFlag: boolean = getFlag();
