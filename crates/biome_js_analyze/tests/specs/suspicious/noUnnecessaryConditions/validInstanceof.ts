// should not generate diagnostics
// The `else` branch of `instanceof` keeps the members that aren't derived
// from the class, and the true branch keeps `unknown` for a generic class,
// whose instance type Biome can't name. A class with the same name and
// members in another namespace is another class.

class Loaded {
	data = { size: 1 };
}

function falseBranch(result: Loaded | string | null) {
	if (!(result instanceof Loaded)) {
		if (result) {
			return 1;
		}
	}
}

function genericClass(value: unknown) {
	if (value instanceof Map) {
		return value?.size;
	}
}

function parentClass(value: Error | string | null) {
	if (value instanceof TypeError) {
		return 1;
	}
	if (value) {
		return 2;
	}
}

namespace First {
	export class Entry {
		size = 1;
	}
}

namespace Second {
	export class Entry {
		size = 1;
	}
}

function sameContents(entry: First.Entry | null) {
	if (!(entry instanceof Second.Entry)) {
		if (entry) {
			return entry.size;
		}
	}
}
