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

function describeAliasedShip(ship: FleetKeys) {
	switch (ship) {
		case "starDestroyers":
			return "Imperial I-class";
		case "tieFighters":
			return "TIE/ln";
		case "shuttles":
			return "Lambda-class";
	}
}

type EscapedKeys = {
	'a"b': unknown;
	'a\\b': unknown;
	'a\nb': unknown;
};

function describeEscapedKey(key: keyof EscapedKeys) {
	switch (key) {
		case "a\"b":
			return "quote";
		case 'a\\b':
			return "backslash";
		case "a\nb":
			return "newline";
	}
}

function genericKey<T>(key: keyof T) {
	switch (key) {
	}
}

type KnownKey = {
	known: unknown;
};

declare function dynamicKey(): string;

type DynamicKey = {
	[dynamicKey()]: unknown;
};

function knownThenDynamicKey(key: keyof KnownKey | keyof DynamicKey) {
	switch (key) {
	}
}

function dynamicThenKnownKey(key: keyof DynamicKey | keyof KnownKey) {
	switch (key) {
	}
}

function knownThenGenericKey<T>(key: keyof KnownKey | keyof T) {
	switch (key) {
	}
}

function constrainedGenericKey<T extends Fleet>(key: keyof T) {
	switch (key) {
	}
}

function numericKey(key: keyof { 1: unknown }) {
	switch (key) {
		case 0x1:
			return true;
	}
}

function hexadecimalKey(key: keyof { 0x1: unknown }) {
	switch (key) {
		case 1:
			return true;
	}
}

function decimalFractionKey(key: keyof { 0.5: unknown }) {
	switch (key) {
		case 0.5:
			return true;
	}
}

function anyKey(key: keyof any) {
	switch (key) {
	}
}

function symbolIndexedKey(key: keyof { [key: symbol]: unknown }) {
	switch (key) {
	}
}

function infiniteNumericKey(key: keyof { 1e999: unknown }) {
	switch (key) {
		case 1e999:
			return true;
	}
}
