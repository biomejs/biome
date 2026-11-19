/* should not generate diagnostics */

const LetterValues = ["A", "B", "C"] as const;
type Letter = (typeof LetterValues)[number];
const Bar = { "A": 1, "B": 2, "C": 3 } as const;
type Letter2 = keyof (typeof Bar);

function exhaustive(letter: Letter) {
	switch (letter) {
		case "A": break;
		case "B": break;
		case "C": break;
	}
}

function exhaustiveKeys(letter: Letter2) {
	switch (letter) {
		case "A": break;
		case "B": break;
		case "C": break;
	}
}

function withDefault(letter: Letter) {
	switch (letter) {
		case "A": break;
		default: break;
	}
}

function literalParameter(letter: "A") {
	switch (letter) {
		case "A": break;
	}
}

declare const extra: Record<string, number>;
const incomplete = { A: 1, ...extra } as const;
function unknownKeys(letter: keyof typeof incomplete) {
	switch (letter) {}
}

function unknownElements(letter: ["A", ...unknown[]][number]) {
	switch (letter) {}
}

function unknownTupleElement(letter: ["A", unknown][number]) {
	switch (letter) {}
}

const mutable = ["A", "B", "C"];
function mutableArray(letter: (typeof mutable)[number]) {
	switch (letter) {}
}

const escaped = { "\u0041": 1, B: 2, C: 3 } as const;
function escapedKeys(letter: keyof typeof escaped) {
	switch (letter) {
		case "A": break;
		case "B": break;
		case "C": break;
	}
}

declare const computedName: "C";
function computedSignature(letter: keyof { A: number; [computedName]: number }) {
	switch (letter) {}
}

function setterSignature(letter: keyof { A: number; set C(value: number) }) {
	switch (letter) {}
}

const escapedValues = ["\u0041", "B", "C"] as const;
function escapedElements(letter: (typeof escapedValues)[number]) {
	switch (letter) {
		case "A": break;
		case "B": break;
		case "C": break;
	}
}

function escapedTupleElements(letter: ["\u0041", "B", "C"][number]) {
	switch (letter) {
		case "A": break;
		case "B": break;
		case "C": break;
	}
}

function controlCharacter(letter: ["A", "\n"][number]) {
	switch (letter) { case "A": break; }
}

function quotedCharacter(letter: ['a"b', "C"][number]) {
	switch (letter) { case "C": break; }
}

function surrogateCharacters(letter: ["\uD800", "\uD801"][number]) {
	switch (letter) { case "\uD800": break; }
}

function brandedEscapedCharacter(letter: ["A", "\u0042" & { brand: true }][number]) {
	switch (letter) { case "A": break; }
}

function quotedKey(letter: keyof { 'a"b': number; C: number }) {
	switch (letter) { case "C": break; }
}

function negativeElement(value: [-1, 2][number]) {
	switch (value) {
		case -1: break;
		case 2: break;
	}
}

const negatives = [-1, 2] as const;
function negativeConstElement(value: (typeof negatives)[number]) {
	switch (value) {
		case -1: break;
		case 2: break;
	}
}

function numericSpellings(value: [0x1, 2, 0, 0.5, 1000][number]) {
	switch (value) {
		case 1: break;
		case +0b10: break;
		case -0: break;
		case .5: break;
		case 1_000: break;
	}
}

function templateElement(value: ["A", `B${number}`][number]) {
	switch (value) { case "A": break; }
}

function infiniteElement(value: [1e400, 2][number]) {
	switch (value) { case 2: break; }
}

function escapedCase(value: ["A", "B", "C"][number]) {
	switch (value) {
		case "\u0041": break;
		case "B": break;
		case "C": break;
	}
}

function escapedOmit(value: keyof Omit<{ A: number; B: number }, "\u0041">) {
	switch (value) { case "B": break; }
}

function largeNumericSpelling(value: [0x1000000000000081, 2][number]) {
	switch (value) {
		case 1152921504606847232: break;
		case 2: break;
	}
}
