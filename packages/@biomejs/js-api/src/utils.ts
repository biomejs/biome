/**
 * Returns how many bytes the UTF-16 code unit would be, if represented in UTF-8.
 * Credit to: https://stackoverflow.com/a/73096001/4668057
 */
function getUtf8ByteLength(codeUnit: string): number {
	const code = codeUnit.charCodeAt(0);
	if (code < 128) {
		return 1;
	}
	if (code < 2048) {
		return 2;
	}
	// UTF-16 high surrogate
	if (55296 <= code && code <= 56319) {
		return 4;
	}
	// UTF-16 low surrogate
	if (56320 <= code && code <= 57343) {
		return 0;
	}
	// Remaining BMP characters (non-surrogates above 2047)
	return 3;
}

/**
 * Converts a span (range) from UTF-8 byte offsets to JavaScript string
 * (UTF-16 code unit) offsets.
 *
 * Biome internally uses UTF-8 byte offsets for source positions, but
 * JavaScript strings use UTF-16 code units. When a source string contains
 * non-ASCII characters (e.g. CJK characters, emoji), the byte offsets
 * returned by Biome will not correspond to the correct positions when
 * used with `String.prototype.slice()` or similar methods.
 *
 * Use this function to convert diagnostic spans before slicing into
 * the original source string.
 *
 * May produce incorrect results if the specified byte span doesn't fall
 * exactly on code unit boundaries.
 *
 * Credit to: https://stackoverflow.com/a/73096001/4668057
 *
 * @example
 * ```ts
 * const result = biome.lintContent(projectKey, source, { filePath: "file.js" });
 * for (const diagnostic of result.diagnostics) {
 *   const [start, end] = spanInBytesToSpanInCodeUnits(
 *     diagnostic.location.span,
 *     source,
 *   );
 *   const text = source.slice(start, end);
 * }
 * ```
 */
export function spanInBytesToSpanInCodeUnits(
	span: [number, number],
	str: string,
): [number, number] {
	const [startInBytes, endInBytes] = span;
	const result: [number, number] = [startInBytes, endInBytes];

	let currCodeUnitIndex = 0;

	// Scan through the string, looking for the start of the substring
	let bytePos = 0;
	while (bytePos < startInBytes && currCodeUnitIndex < str.length) {
		const byteLength = getUtf8ByteLength(str.charAt(currCodeUnitIndex));
		bytePos += byteLength;
		++currCodeUnitIndex;

		// Make sure to include low surrogate
		if (byteLength === 4 && bytePos === startInBytes) {
			++currCodeUnitIndex;
		}
	}

	// We've found the start
	result[0] = currCodeUnitIndex;

	// Now scan through the following string to find the end
	while (bytePos < endInBytes && currCodeUnitIndex < str.length) {
		const byteLength = getUtf8ByteLength(str.charAt(currCodeUnitIndex));
		bytePos += byteLength;
		++currCodeUnitIndex;

		// Make sure to include low surrogate
		if (byteLength === 4 && bytePos === endInBytes) {
			++currCodeUnitIndex;
		}
	}

	// We've found the end
	result[1] = currCodeUnitIndex;

	return result;
}
