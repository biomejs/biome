interface I {
	known: string;

	[key: string]: string;
}

const a: I = {
	known: "",
	unknown: "",
}

// Valid
a.known;

// Invalid
a["known"];

// Ignored
a.unknown;
a["unknown"];
