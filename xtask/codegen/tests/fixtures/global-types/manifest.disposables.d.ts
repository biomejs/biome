interface Error {
    name: string;
    message: string;
    stack?: string;
}

interface ErrorConstructor {
    new(message?: string): Error;
    (message?: string): Error;
    readonly prototype: Error;
}

declare var Error: ErrorConstructor;

interface Symbol {}

interface SymbolConstructor {
    readonly prototype: Symbol;
}

interface SymbolConstructor {
    readonly [Symbol.iterator]: unique symbol;
    readonly dispose: unique symbol;
    readonly asyncDispose: unique symbol;
}

declare var Symbol: SymbolConstructor;

interface Disposable {
    [Symbol.dispose](): void;
}

interface AsyncDisposable {
    [Symbol.asyncDispose](): PromiseLike<void>;
}

interface Array<T> {
    length: number;
    filter(predicate: (value: T, index: number, array: T[]) => unknown, thisArg?: any): T[];
    forEach(callbackfn: (value: T, index: number, array: T[]) => void, thisArg?: any): void;
    map<U>(callbackfn: (value: T, index: number, array: T[]) => U, thisArg?: any): U[];
}

interface RegExpExecArray {}

interface RegExp {
    exec(string: string): RegExpExecArray | null;
}

interface Date {
    toString(): string;
}

interface Date {
    valueOf(): number;
}

interface Map<K, V> {}

interface Set<T> {}

interface WeakMap<K, V> {}
