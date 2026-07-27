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
