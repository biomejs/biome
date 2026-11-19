// should generate diagnostics
// The declaration file types `kindOf` as `typeof` a function without a return
// type annotation, so a call of it returns what TypeScript infers for that
// function: every result of `typeof`. The `switch` misses cases.

import { kindOf } from "./declarations";

type Kind =
	| "bigint"
	| "boolean"
	| "function"
	| "number"
	| "object"
	| "string"
	| "symbol"
	| "undefined";

export function describe(value: string) {
	let kind: Kind;
	kind = kindOf(value);
	switch (kind) {
		case "string":
			return "text";
	}
}
