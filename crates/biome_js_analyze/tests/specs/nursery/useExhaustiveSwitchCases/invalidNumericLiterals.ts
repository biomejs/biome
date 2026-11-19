/* should generate diagnostics */

import type { Missing } from "./numericLiteralsTypes";

function distinctMissingValues(value: Missing) {
	switch (value) {
		case -1: break;
	}
}

function distinctLargeValues(value: 0x1000000000000081 | 0x1000000000000000) {
	switch (value) {
		case 1152921504606846976: break;
	}
}

function equivalentFractions(value: 0.5 | .50 | 5e-1 | 1) {
	switch (value) {
		case 1: break;
	}
}
