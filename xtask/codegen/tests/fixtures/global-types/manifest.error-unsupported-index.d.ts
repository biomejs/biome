interface Error {
    name: string;
    message: string;
    stack?: string;
    [key: string]: string;
}

interface ErrorConstructor {
    new(message?: string): Error;
    (message?: string): Error;
    readonly prototype: Error;
}

declare var Error: ErrorConstructor;
