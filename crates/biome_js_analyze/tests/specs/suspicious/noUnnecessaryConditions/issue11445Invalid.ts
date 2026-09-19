/* should generate diagnostics */

type Fleet = {
	starDestroyers: number;
	tieFighters: number;
	shuttles: number;
};

function describeInvalidShip(ship: keyof Fleet) {
	switch (ship) {
		case "starDestroyers":
			return "Imperial I-class";
		case "notAShip":
			return "invalid";
	}
}

type FleetKeys = keyof Fleet;

function describeInvalidAlias(ship: FleetKeys) {
	switch (ship) {
		case "unknownShip":
			return "invalid";
	}
}

type Common = keyof ({ left: number; shared: string } | { right: number; shared: string });

function describeInvalidUnion(key: Common) {
	switch (key) {
		case "left":
			return "invalid";
	}
}

type Combined = keyof ({ left: number } & { right: number });

function describeInvalidIntersection(key: Combined) {
	switch (key) {
		case "left":
			return "left";
		case "right":
			return "right";
		case "other":
			return "invalid";
	}
}

type NumericKey = { 1: unknown };

function describeInvalidNumericKey(key: keyof NumericKey) {
	switch (key) {
		case "1":
			return "invalid";
	}
}

type QuotedKey = { "1": unknown };

function describeInvalidQuotedKey(key: keyof QuotedKey) {
	switch (key) {
		case 1:
			return "invalid";
	}
}

type MixedKeys = keyof ({ 1: number } & { "1": string });

function describeInvalidMixedKey(key: MixedKeys) {
	switch (key) {
		case 1:
		case "1":
			return "valid";
		case "missing":
			return "invalid";
	}
}
