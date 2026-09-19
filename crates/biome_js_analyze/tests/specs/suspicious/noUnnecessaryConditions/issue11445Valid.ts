/* should not generate diagnostics */

type Fleet = {
	starDestroyers: number;
	tieFighters: number;
	shuttles: number;
};

type FleetKeys = keyof Fleet;

function describeShip(ship: keyof Fleet) {
	switch (ship) {
		case "starDestroyers":
			return "Imperial I-class";
		case "tieFighters":
			return "TIE/ln";
		case "shuttles":
			return "Lambda-class";
	}
}

function isCapitalShip(ship: FleetKeys) {
	switch (ship) {
		case "starDestroyers":
			return true;
	}
	return false;
}

type NumericAndQuoted = {
	1: number;
	"quoted": string;
};

function isNumericOrQuoted(key: keyof NumericAndQuoted) {
	switch (key) {
		case 1:
			return "numeric";
		case "quoted":
			return "quoted";
	}
}

type NumericFirst = keyof ({ 1: number } & { "1": string });

function isNumericFirst(key: NumericFirst) {
	switch (key) {
		case 1:
		case "1":
			return true;
	}
}

type StringFirst = keyof ({ "1": string } & { 1: number });

function isStringFirst(key: StringFirst) {
	switch (key) {
		case 1:
		case "1":
			return true;
	}
}

type EscapedIdentifier = {
	\u0061: unknown;
};

function isEscapedIdentifier(key: keyof EscapedIdentifier) {
	switch (key) {
		case "a":
			return true;
	}
}

function isLargeHexNumericKey(key: keyof { 0x10000000000000000: unknown }) {
	switch (key) {
		case 18446744073709551616:
			return true;
	}
}

type Common = keyof ({ left: number; shared: string } | { right: number; shared: string });

function isCommon(key: Common) {
	switch (key) {
		case "shared":
			return true;
	}
}

type Combined = keyof ({ left: number } & { right: number });

function isCombined(key: Combined) {
	switch (key) {
		case "left":
		case "right":
			return true;
	}
}

type NumberIndexed = { [key: number]: string };

function isNumberIndex(key: keyof NumberIndexed) {
	switch (key) {
		case 1:
			return true;
	}
}

type StringIndexed = { [key: string]: string };

function isStringIndex(key: keyof StringIndexed) {
	switch (key) {
		case "known":
		case 1:
			return true;
	}
}

type Base = { inherited: number };
interface Derived extends Base {
	own?: string;
}

function isDerivedKey(key: keyof Derived) {
	switch (key) {
		case "inherited":
		case "own":
			return true;
	}
}

type Recursive = { next?: Recursive; value: number };

function isRecursiveKey(key: keyof Recursive) {
	switch (key) {
		case "next":
		case "value":
			return true;
	}
}

function genericKey<T>(key: keyof T) {
	switch (key) {
		case "unknown":
			return true;
	}
	return false;
}

function constrainedGenericKey<T extends Fleet>(key: keyof T) {
	switch (key) {
		case "starDestroyers":
			return true;
	}
	return false;
}

function unknownKey(key: keyof unknown) {
	switch (key) {
	}
	return key;
}

type MissingKeys = keyof MissingType;

function incompleteKey(key: MissingKeys) {
	switch (key) {
		case "unknown":
			return true;
	}
	return false;
}
