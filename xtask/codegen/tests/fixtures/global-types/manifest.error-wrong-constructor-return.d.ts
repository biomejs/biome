interface Error {
    name: string;
}

interface Error {
    message: string;
    stack?: string;
}

interface ErrorConstructor {
    new(message?: string): void;
    (message?: string): Error;
    readonly prototype: Error;
}

declare var Error: ErrorConstructor;

interface Symbol {}

interface SymbolConstructor {
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
