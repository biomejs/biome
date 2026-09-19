/* should generate diagnostics */

type Fleet = {
	starDestroyers: number;
	tieFighters: number;
	shuttles: number;
};

function describeShip(ship: keyof Fleet) {
	switch (ship) {
		case "starDestroyers":
			return "Imperial I-class";
		case "tieFighters":
			return "TIE/ln";
	}
}

type EscapedMissingKeys = {
	'a"b': unknown;
	'a\\b': unknown;
	'a\nb': unknown;
};

function describeMissingEscapedKey(key: keyof EscapedMissingKeys) {
	switch (key) {
	}
}
