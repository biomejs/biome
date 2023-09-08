function printArg<T = void>(arg: T) {}
logAndReturn<void>(undefined);

let voidPromise: Promise<void> = new Promise<void>(() => { });
let voidMap: Map<string, void> = new Map<string, void>();
