type Single = { a: number };
type Multiple = { a: number; b: string };
type Empty = {};
type ExpandedSingle = {
  a: number;
};
type Nested = { a: { b: number; c: string } };
type Methods = { a(): void; b: string };
type SingleTuple = [number];
type MultipleTuple = [number, string];
type EmptyTuple = [];
type ExpandedSingleTuple = [
  number,
];
type NamedTuple = [a: number, b: string];
type RestTuple = [number, ...string[]];
type NestedTuple = [{ a: number; b: string }];
const singleParameter = ({ a }: { a: number }) => a;
const multipleParameters = ({ a, b }: { a: number; b: number }) => a + b;
function consume({ a, b }: { a: number; b: number }) {}
type Comments = { /* first */ a: number; /* between */ b: string /* last */ };
type TupleComments = [/* first */ number, /* between */ string /* last */];
