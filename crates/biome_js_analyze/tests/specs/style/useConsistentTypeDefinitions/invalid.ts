// Default: prefer interface
type Foo = {
    prop: string;
};

type Bar = {
    method(): void;
};

type Point = { x: number; y: number; };

type Nested = { x: {one: string; two: number}; y: {one: string; two: number}; };

type User = {
    name: string;
    age: number;
};

type Config = {
    apiUrl: string;
    timeout: number;
    retries: number;
};

type Comments /* comment 1 */ = /* comment 2 */ { /* comment 3 */
    /* comment 4 */ lorem: /* comment 5 */ boolean; /* comment 6 */
/* comment 7 */ };

// These should be valid and not trigger errors
type Union = { a: string } | { b: number };
type Intersection = { a: string } & { b: number };
type Conditional<T> = T extends string ? { a: string } : { b: number };
interface ValidInterface {
    prop: string;
}