// should not generate diagnostics
import type { Letters } from "./issue12026Letters.ts";
import { Letters as letters } from "./issue12026Letters.ts";
import * as letterModule from "./issue12026Letters.ts";

function fromType(letter: Letters): number {
	switch (letter) {
		case "a":
			return 1;
		case "b":
			return 2;
		case "c":
			return 3;
	}
}

function fromValue(letter: (typeof letters)[number]): number {
	switch (letter) {
		case "a":
			return 1;
		case "b":
			return 2;
		case "c":
			return 3;
	}
}

function fromNamespace(letter: letterModule.Letters): number {
	switch (letter) {
		case "a":
			return 1;
		case "b":
			return 2;
		case "c":
			return 3;
	}
}
