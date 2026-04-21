interface SuppressionsWithTrailingComments {
	value:
		// biome-ignore format: preserve the nested type formatting
		Array<string|number>; // keep this trailing comment after the suppressed type
	done: void;
}
