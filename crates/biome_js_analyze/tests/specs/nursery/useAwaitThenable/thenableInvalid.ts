/* should generate diagnostics */

// A non-callable `then` member does not make a thenable, so `await` is still reported.
interface NotThenable { then: number; }
declare const value: unknown;
async function awaitNonCallableThen(): Promise<void> {
    await (value as NotThenable);
}
