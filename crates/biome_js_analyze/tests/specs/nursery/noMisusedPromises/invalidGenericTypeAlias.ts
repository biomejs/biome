// `MaybePromise<string>` substitutes to `string | Promise<string>`, so a Promise reaches each condition.
type MaybePromise<T> = T | Promise<T>;
declare const cached: MaybePromise<string>;
if (cached) {
}
const label = cached ? "ready" : "pending";
while (cached) {
}
