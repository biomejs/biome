/* should not generate diagnostics */
const a = new Set([1, 2]);
const b = new Set([2, 3]);
let mutable = new Set([3]);

// The rule can't tell whether these values are Sets.
const unknownUnion = new Set([...first, ...second]);
const unknownFilter = [...a].filter((value) => second.has(value));
const unknownReceiver = [...first].filter((value) => b.has(value));
const reassignable = [...a].filter((value) => mutable.has(value));
const mixed = new Set([...a, ...[1, 2]]);
const map = new Map();
const mapFilter = [...a].filter((value) => map.has(value));

// Not a union of Sets only.
const single = new Set([...a]);
const extra = new Set([...a, ...b, 4]);
const notGlobal = (() => {
	const Set = class {};
	return new Set([...a, ...b]);
})();

// The spread operand may have side effects.
const sideEffect = new Set([...a, ...(update(), b)]);
const constructedOperand = new Set([...new Set([1]), ...b]);

// The callback doesn't only check membership.
const large = [...a].filter((value) => value > 1);
const asyncCallback = [...a].filter(async (value) => b.has(value));
const functionCallback = [...a].filter(function (value) { return b.has(value); });
const blockBody = [...a].filter((value) => { return b.has(value); });
const twoParameters = [...a].filter((value, index) => b.has(value));
const defaultParameter = [...a].filter((value = 1) => b.has(value));
const destructured = [...a].filter(({ value }) => b.has(value));
const usedTwice = [...a].filter((value) => b.has(value) && value > 1);
const otherArgument = [...a].filter((value) => b.has(other));
const optionalCall = [...a].filter((value) => b.has?.(value));
const optionalMember = [...a].filter((value) => b?.has(value));
const computed = [...a].filter((value) => b["has"](value));
const sideEffectOther = [...a].filter((value) => getSet().has(value));
const notHas = [...a].filter((value) => b.delete(value));

// The array contains more than a single Set.
const twoSets = [...a, ...b].filter((value) => b.has(value));
const notSpread = [a].filter((value) => b.has(value));

// The result of filter is used as an array.
const mapped = [...a].filter((value) => b.has(value)).map(String);
const length = [...a].filter((value) => b.has(value)).length;
const argument = format([...a].filter((value) => b.has(value)));
const notSetConstructor = new Array([...a].filter((value) => b.has(value)));

// Optional and computed array methods.
const optionalFilter = [...a]?.filter((value) => b.has(value));
const computedFilter = [...a]["filter"]((value) => b.has(value));
const findCall = [...a].find((value) => b.has(value));

// Size comparisons that don't check for an empty Set.
const one = a.intersection(b).size === 1;
const loose = a.intersection(b).size == 0;
const greater = 0 > a.intersection(b).size;
const less = a.intersection(b).size < 0;
const length2 = a.intersection(b).length === 0;
const union = a.union(b).size === 0;
const optionalSize = a.intersection(b)?.size === 0;
const unknownSize = a.intersection(second).size === 0;

// Already using the Set methods.
const combined = a.union(b);
const shared = a.intersection(b);
const isSubset = a.isSubsetOf(b);
const isDisjoint = a.isDisjointFrom(b);
