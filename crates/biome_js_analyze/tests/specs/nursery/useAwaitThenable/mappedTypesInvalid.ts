/* should generate diagnostics */

// Awaiting a non-thenable from a mapped type
type StringValues<T> = {
    [K in keyof T]: string;
};

interface Config {
    name: string;
    count: number;
}

declare const values: StringValues<Config>;
await values.name;
