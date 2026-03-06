// should generate diagnostics

// Direct interface member access returning Promise
interface SimpleApi {
    getThing: () => Promise<string>;
}
declare const api: SimpleApi;
api.getThing();

// Concrete mapped type (no generics)
type ConcreteMapping = {
    [K in "foo" | "bar"]: () => Promise<string>;
};
declare const concrete: ConcreteMapping;
concrete.foo();

// Generic mapped type (simple, no key remapping)
type AsyncActions<T> = {
    [K in keyof T]: () => Promise<T[K]>;
};
interface Config {
    name: string;
    count: number;
}
declare const actions: AsyncActions<Config>;
actions.name();

// Key remapping with template literals (issue #6603 scenario)
interface Things {
    thing: string;
}
type CalculateGetter<T> = {
    [K in keyof T as K extends string ? `get_${K}` : never]: () => Promise<T[K]>;
};
declare const lazyThings: CalculateGetter<Things>;
lazyThings.get_thing();
