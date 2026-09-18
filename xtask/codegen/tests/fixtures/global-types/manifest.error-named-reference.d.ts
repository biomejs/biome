interface ErrorOptions {
    cause?: unknown;
}

interface Error {
    name: string;
    message: ErrorOptions;
    stack?: string;
}

interface ErrorConstructor {
    new(message?: string): Error;
    (message?: string): Error;
    readonly prototype: Error;
}

declare var Error: ErrorConstructor;
