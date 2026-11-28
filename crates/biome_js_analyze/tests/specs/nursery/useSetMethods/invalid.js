/* should generate diagnostics */
const a = new Set([1, 2]);
const b = new Set([2, 3]);
const c = new Set([3, 4]);

// Union
const union1 = new Set([...a, ...b]);
const union2 = new Set([...a, ...b, ...c]);
const union4 = new Set([...(cond ? a : b), ...c]);
const union5 = new Set(([...a, ...b]));

// Intersection and difference
const intersection1 = [...a].filter((value) => b.has(value));
const intersection2 = [...a].filter(value => b.has(value));
const intersection3 = new Set([...a].filter((value) => b.has(value)));
const difference1 = [...a].filter((value) => !b.has(value));
const difference2 = new Set([...a].filter((value) => !b.has(value)));
const difference3 = [...a].filter((value) => !(b.has(value)));
const intersection4 = new Set([...a].filter((value) => b.has(value))).size;

// Predicates
const subset1 = [...a].every((value) => b.has(value));
const overlaps1 = [...a].some((value) => b.has(value));
const disjoint1 = [...a].every((value) => !b.has(value));
const notSubset1 = [...a].some((value) => !b.has(value));
if ([...a].some((value) => b.has(value))) {}
const asString = [...a].some((value) => b.has(value)).toString();
const power = [...a].some((value) => b.has(value)) ** 2;

// Size comparisons
const disjoint2 = a.intersection(b).size === 0;
const disjoint3 = 0 === a.intersection(b).size;
const overlaps2 = a.intersection(b).size !== 0;
const overlaps3 = a.intersection(b).size > 0;
const overlaps4 = 0 < a.intersection(b).size;
const subset2 = a.difference(b).size === 0;
const notSubset2 = a.difference(b).size !== 0;
const subset3 = (a.difference(b)).size === 0;

// Keyword directly before the reported code
function keyword() {
	return[...a].filter((value) => b.has(value));
}

// Comments are not removed by the fix
const withComment = new Set([...a, /* comment */ ...b]);
const operandComment = new Set([...a, .../* comment */ b]);

// Receivers that need parentheses
const newReceiver = new Set([...new Set(), ...b]);
const newReceiverWithoutParentheses = new Set([...new Set, ...b]);
const conditionalReceiver = [...(cond ? a : b)].every((value) => c.has(value));

// Unparenthesized `new` callee
const constructed = new new Set([...a, ...b]).constructor();
