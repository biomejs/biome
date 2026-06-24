// `Id<string>` substitutes to `string`, so the indexOf comparison can use includes().
type Id<T> = T;
declare const text: Id<string>;
text.indexOf("a") !== -1;
