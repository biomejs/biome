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
