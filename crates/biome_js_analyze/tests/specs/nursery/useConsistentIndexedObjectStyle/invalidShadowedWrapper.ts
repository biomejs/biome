/* should generate diagnostics */
function local() {
	type Partial<T> = { [key: string]: T };
	type Optional = { [key in string]?: number };
}

type OutsideShadowingScope = { [key in string]?: number };
