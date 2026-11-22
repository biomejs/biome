/* should generate diagnostics */

declare const UNDEFINED_VOID_ONLY: unique symbol;
type VoidOrUndefinedOnly = void | { [UNDEFINED_VOID_ONLY]: never };

export namespace ReactLike {
    export function act(callback: () => VoidOrUndefinedOnly): void;
    export function act<T>(callback: () => T | Promise<T>): Promise<T>;
}

// The void-returning overload is selected for a synchronous callback.
export async function awaitVoidOverload(): Promise<void> {
    await ReactLike.act(() => {});
}
