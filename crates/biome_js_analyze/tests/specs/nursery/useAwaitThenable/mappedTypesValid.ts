/* should not generate diagnostics */

// Awaiting a Promise from a mapped type is correct
type Promisified<T> = {
    [K in keyof T]: Promise<T[K]>;
};

interface Config {
    name: string;
    count: number;
}

declare const values: Promisified<Config>;
await values.name;
