/* should not generate diagnostics */

// Mirrors React's `act` overloads: the leading signature only accepts
// callbacks that return `void`, so an async callback selects the Promise
// overload declared inside the namespace.
declare const UNDEFINED_VOID_ONLY: unique symbol;
type VoidOrUndefinedOnly = void | { [UNDEFINED_VOID_ONLY]: never };

export namespace ReactLike {
    export function act(callback: () => VoidOrUndefinedOnly): void;
    export function act<T>(callback: () => T | Promise<T>): Promise<T>;
}

export async function awaitNamespaceOverload(): Promise<void> {
    await ReactLike.act(async () => {});
}
