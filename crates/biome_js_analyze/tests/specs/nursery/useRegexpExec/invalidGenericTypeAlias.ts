// `Id<string>` substitutes to `string`, so the String#match() call can use RegExp#exec().
type Id<T> = T;
declare const text: Id<string>;
text.match(/foo/);
