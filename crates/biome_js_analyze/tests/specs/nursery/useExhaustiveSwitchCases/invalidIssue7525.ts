interface test {
	a?: number;
	b?: "thing" | "other" | "more";
}

function fn(param: number, { a = 1, b = "thing" }: test = {}) {
	// incorrectly flagged as not exhaustive
	// even though `b` is never `undefined`
	// due to having a default value
	switch (b) {
		case "thing":
			return 1;
		case "other":
			return 2;
		case "more":
			return 3;
	}
}
