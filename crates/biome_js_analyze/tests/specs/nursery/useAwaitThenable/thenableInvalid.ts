/* should generate diagnostics */

// A non-callable `then` member does not make a thenable, so `await` is still reported.
interface NotThenable { then: number; }
class PromiseLike<T> {}
class InstanceThen { then() {} }
class Other {}
declare const value: unknown;
declare const promiseLike: PromiseLike<void>;
declare const constructors: typeof InstanceThen | typeof Other;
async function awaitNonCallableThen(): Promise<void> {
    await (value as NotThenable);
    await Promise;
    await promiseLike;
    await constructors;
}
