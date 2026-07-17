// `Id<string>` substitutes to `string`, so these index comparisons can use startsWith/endsWith.
type Id<T> = T;
declare const text: Id<string>;
text[0] === "a";
text[text.length - 1] === "z";
