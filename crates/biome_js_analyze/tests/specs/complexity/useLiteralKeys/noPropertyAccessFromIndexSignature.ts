interface I {
	known: string;

	[key: string]: string;
}

const a: I = {
	known: "",
	unknown: "",
}

// Valid
a.known
a["unknown"];

// Invalid
a["known"];
a.unknown;

