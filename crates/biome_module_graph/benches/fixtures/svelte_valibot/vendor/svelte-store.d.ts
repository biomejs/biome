export type Subscriber<T> = (value: T) => void;
export type Unsubscriber = () => void;
export type Updater<T> = (value: T) => T;
export type StartStopNotifier<T> = (set: (value: T) => void, update: (fn: Updater<T>) => void) => void | (() => void);
export interface Readable<T> {
    subscribe(this: void, run: Subscriber<T>, invalidate?: () => void): Unsubscriber;
}
export interface Writable<T> extends Readable<T> {
    set(this: void, value: T): void;
    update(this: void, updater: Updater<T>): void;
}
export function toStore<V>(get: () => V, set: (v: V) => void): Writable<V>;
export function toStore<V>(get: () => V): Readable<V>;
export function fromStore<V>(store: Writable<V>): {
    current: V;
};
export function fromStore<V>(store: Readable<V>): {
    readonly current: V;
};
export function readable<T>(value?: T | undefined, start?: StartStopNotifier<T> | undefined): Readable<T>;
export function writable<T>(value?: T | undefined, start?: StartStopNotifier<T> | undefined): Writable<T>;
export function derived<S extends Stores, T>(stores: S, fn: (values: StoresValues<S>, set: (value: T) => void, update: (fn: Updater<T>) => void) => Unsubscriber | void, initial_value?: T | undefined): Readable<T>;
export function derived<S extends Stores, T>(stores: S, fn: (values: StoresValues<S>) => T, initial_value?: T | undefined): Readable<T>;
export function readonly<T>(store: Readable<T>): Readable<T>;
export function get<T>(store: Readable<T>): T;
type Stores = Readable<any> | [
    Readable<any>,
    ...Array<Readable<any>>
] | Array<Readable<any>>;
type StoresValues<T> = T extends Readable<infer U> ? U : {
    [K in keyof T]: T[K] extends Readable<infer U> ? U : never;
};
export {};
