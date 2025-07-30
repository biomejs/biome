// Option: prefer type
interface Foo {
    prop: string;
}

interface Bar {
    method(): void;
}

interface Point {
    x: number;
    y: number;
}

interface User {
    name: string;
    age: number;
}

interface Config {
    apiUrl: string;
    timeout: number;
    retries: number;
}

// These should not trigger errors as interfaces cannot represent these
type Union = { a: string } | { b: number };
type Intersection = { a: string } & { b: number };
type StringAlias = string;