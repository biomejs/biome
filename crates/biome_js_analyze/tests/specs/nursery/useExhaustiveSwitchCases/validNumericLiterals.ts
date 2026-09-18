/* should not generate diagnostics */

import type { Large, Small } from "./numericLiteralsTypes";

function equivalentSpellings(value: Small) {
	switch (value) {
		case -0x1: break;
		case -0: break;
		case 1: break;
		case +2: break;
		case 3: break;
		case .5: break;
		case 1_000: break;
	}
}

function largeRadixLiteral(value: Large) {
	switch (value) {
		case 1152921504606847232: break;
		case 0b10: break;
	}
}

function roundedDuplicates(value: 0x20000000000001 | 9007199254740992) {
	switch (value) {
		case 9007199254740992: break;
	}
}

function radixOverflow(value: 0x100000000000000000000000000000000 | 2) {
	switch (value) {
		case 2: break;
	}
}

function decimalOverflow(value: 1e400 | 2) {
	switch (value) {
		case 2: break;
	}
}

function unsupportedCase(value: 1 | 2) {
	switch (value) {
		case 0x100000000000000000000000000000000: break;
	}
}
