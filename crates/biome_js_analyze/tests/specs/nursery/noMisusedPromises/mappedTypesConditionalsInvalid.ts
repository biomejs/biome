/* should generate diagnostics */

// Mapped type producing Promise members used in conditional
type Promisified<T> = {
    [K in keyof T]: Promise<T[K]>;
};

interface Data {
    value: string;
}

declare const promisified: Promisified<Data>;

if (promisified.value) {
    // Do something
}
