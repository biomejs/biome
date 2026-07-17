// `Id<Disposable>` substitutes to `Disposable`, so the result should be held with `using`.
type Id<T> = T;
declare function open(): Id<Disposable>;
const handle = open();
