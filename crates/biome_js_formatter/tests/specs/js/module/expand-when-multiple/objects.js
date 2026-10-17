const single = { a: 1 };
const multiple = { a: 1, b: 2 };
const empty = {};
const expandedSingle = {
  a: 1,
};
const expandedMultiple = {
  a: 1, b: 2,
};
const nested = { a: { b: 1, c: 2 } };
const shorthand = { a, b };
const spread = { ...other, a: 1 };
const method = { a() {}, b: 2 };
const longSingle = { value: firstArgument + secondArgument + thirdArgument + fourthArgument + fifthArgument };
const comments = { /* first */ a: 1, /* between */ b: 2 /* last */ };
const singleComment = { a: 1 /* last */ };
const lineComment = { a: 1 // last
};
const emptyComment = { /* empty */ };
consume({ a: 1, b: 2 });
const callback = () => ({ a: 1, b: 2 });
// biome-ignore format: Preserve the explicit layout
const suppressed = { a: 1, b: 2 };
