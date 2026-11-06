// Parse default-theme color values into the components that the Rust
// `color` crate's `parse_color` produces, so the generator can emit them as
// `color::DynamicColor` constants.
//
// Only the syntaxes used by Tailwind's theme.css are accepted: `oklch()`
// without alpha, and hex. Anything else throws, so a theme.css that starts
// using another syntax fails codegen instead of emitting a wrong color.
//
// Scaling mirrors `parse_color` in
// https://github.com/linebender/color/blob/main/color/src/parse.rs:
// oklch lightness percentages are scaled by 0.01 and clamped to [0, 1],
// chroma percentages by 0.004 and clamped to >= 0, and hex channels are
// divided by 255.

export type ThemeColorSpace = "Oklch" | "Srgb";

export type ParsedThemeColor = {
	space: ThemeColorSpace;
	// `null` marks a `none` (missing) component.
	components: [number | null, number | null, number | null, number];
};

const OKLCH = /^oklch\(\s*(\S+)\s+(\S+)\s+(\S+)\s*\)$/;
const HEX = /^#([0-9a-f]{3,4}|[0-9a-f]{6}|[0-9a-f]{8})$/i;
const NUMBER = /^[+-]?(\d+(\.\d+)?|\.\d+)$/;

export function parseThemeColor(value: string): ParsedThemeColor {
	const oklch = value.match(OKLCH);
	if (oklch) {
		const l = component(value, oklch[1], 0.01);
		const c = component(value, oklch[2], 0.004);
		const h = component(value, oklch[3], null);
		return {
			space: "Oklch",
			components: [
				l === null ? null : Math.min(Math.max(l, 0), 1),
				c === null ? null : Math.max(c, 0),
				h,
				1,
			],
		};
	}

	const hex = value.match(HEX);
	if (hex) {
		let digits = hex[1];
		if (digits.length <= 4) {
			digits = [...digits].map((d) => d + d).join("");
		}
		if (digits.length === 6) digits += "ff";
		const [r, g, b, a] = [0, 2, 4, 6].map(
			(i) => Number.parseInt(digits.slice(i, i + 2), 16) / 255,
		);
		return { space: "Srgb", components: [r, g, b, a] };
	}

	throw new Error(`unsupported theme color syntax: ${value}`);
}

// Parses a number, a percentage (when `pctScale` is non-null), or `none`.
function component(
	value: string,
	token: string,
	pctScale: number | null,
): number | null {
	if (token === "none") return null;
	if (pctScale !== null && token.endsWith("%")) {
		const n = token.slice(0, -1);
		if (NUMBER.test(n)) return Number(n) * pctScale;
	} else if (NUMBER.test(token)) {
		return Number(token);
	}
	throw new Error(
		`unsupported component \`${token}\` in theme color: ${value}`,
	);
}
