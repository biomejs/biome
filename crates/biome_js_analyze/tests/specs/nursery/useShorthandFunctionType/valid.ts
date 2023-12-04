type Example = () => string;

function foo(example: () => number): number {
 return bar();
}

// returns the function itself, not the `this` argument.
type ReturnsSelf = (arg: string) => ReturnsSelf;

interface Foo {
 bar: string;
}

interface Bar extends Foo {
 (): void;
}

// multiple call signatures (overloads) is allowed:
interface Overloaded {
 (data: string): number;
 (id: number): string;
}

// this is equivelent to Overloaded interface.
type Intersection = ((data: string) => number) & ((id: number) => string);

interface ReturnsSelf {
 // returns the function itself, not the `this` argument.
 (arg: string): this;
}