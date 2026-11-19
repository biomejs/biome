interface BaseErrorConstructor {
    readonly captureStackTrace: Error;
}

interface Error {
    name: string;
    message: string;
    stack?: string;
}

interface ErrorConstructor extends BaseErrorConstructor {
    new(message?: string): Error;
    (message?: string): Error;
    readonly prototype: Error;
}

declare var Error: ErrorConstructor;
