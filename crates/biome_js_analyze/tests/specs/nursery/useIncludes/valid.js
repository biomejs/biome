// should not generate diagnostics
const arr = [1, 2, 3];
const str = "hello world";

// already using includes
arr.includes(1);
str.includes("world");
!arr.includes(1);

// positional use of indexOf — result stored, not compared
const pos = arr.indexOf(1);

// indexOf with fromIndex argument — semantics differ, leave alone
arr.indexOf(1, 2) !== -1;
arr.indexOf(1, -1) !== -1;

// unrelated comparisons that happen to use indexOf result
arr.indexOf(1) > 0;
arr.indexOf(1) >= 1;
arr.indexOf(1) === 0;
arr.indexOf(1) < -1;

// indexOf result used in arithmetic
arr.indexOf(1) + 1;

// not a member call at all
indexOf(1) !== -1;

// lastIndexOf with fromIndex or positional use
const last = arr.lastIndexOf(1);
arr.lastIndexOf(1, 2) !== -1;
arr.lastIndexOf(1) > 0;

// some() callbacks that includes() cannot replace
arr.some(x => x > 1);
arr.some(x => x == 1);
arr.some(x => x === x);
arr.some((x, i) => x === i);
arr.some((x = 1) => x === 1);
arr.some(x => { log(x); return x === 1; });
arr.some(async x => x === 1);
arr.some(isOne);
arr.some(x => x === 1, thisArg);

// some() is not defined on strings; presence checks on unknown types
unknownValue.some(x => x === 1);
