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

// positional use of lastIndexOf — result stored, not compared
const lastPos = arr.lastIndexOf(1);

// lastIndexOf with fromIndex argument — semantics differ, leave alone
arr.lastIndexOf(1, 2) !== -1;

// unrelated comparisons that happen to use lastIndexOf result
arr.lastIndexOf(1) > 0;
arr.lastIndexOf(1) >= 1;
arr.lastIndexOf(1) === 0;

// lastIndexOf result used in arithmetic
arr.lastIndexOf(1) + 1;

// not a member call at all
lastIndexOf(1) !== -1;
