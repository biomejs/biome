// `Id<string>` substitutes to the non-nullish `string`, so the optional chain is unnecessary.
type Id<T> = T;
function readLength(text: Id<string>) {
	return text?.length;
}
