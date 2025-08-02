/* should not generate diagnostics */
// Option: prefer type
type Foo = {
    prop: string;
};

type Bar = {
    method(): void;
};

type Point = {
    x: number;
    y: number;
};

type User = {
    name: string;
    age: number;
};

type Config = {
    apiUrl: string;
    timeout: number;
    retries: number;
};

// Also valid for other type constructs
type Union = { a: string } | { b: number };
type Intersection = { a: string } & { b: number };
type StringAlias = string;
type FunctionType = (x: number) => string;
type TupleType = [string, number];