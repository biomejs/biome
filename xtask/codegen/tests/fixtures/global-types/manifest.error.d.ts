interface Error {
    name: string;
}

interface Error {
    message: string;
    stack?: string;
}

interface ErrorConstructor {
    new(message?: string): Error;
    (message?: string): Error;
    readonly prototype: Error;
}

declare var Error: ErrorConstructor;

declare global {
    interface TypeError extends Error {
        code?: string;
    }
}

declare module "external" {
    interface ExternalError {
        ignored: string;
    }
}
