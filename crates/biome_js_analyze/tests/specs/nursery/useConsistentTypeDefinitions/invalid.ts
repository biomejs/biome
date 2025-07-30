// Default: prefer interface
type Foo = {
    prop: string;
};

type Bar = {
    method(): void;
};

type Point = { x: number; y: number; };

type User = {
    name: string;
    age: number;
};

type Config = {
    apiUrl: string;
    timeout: number;
    retries: number;
};

// These should be valid and not trigger errors
type Union = { a: string } | { b: number };
type Intersection = { a: string } & { b: number };
type Conditional<T> = T extends string ? { a: string } : { b: number };
interface ValidInterface {
    prop: string;
}