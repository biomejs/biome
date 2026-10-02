/* should not generate diagnostics */

function nullable(value: ["A", null][number]) {
	if (value) {}
	return value ?? "A";
}

function optional(value: ["A"?][number]) {
	return value?.length;
}

function unknown(value: [unknown][number]) {
	if (value) {}
}

declare const computedName: "C";
type Incomplete = { A: number; [computedName]: number };
function readonlyKey(key: keyof Readonly<Incomplete>) {
	switch (key) { case "C": break; }
}

function partialKey(key: keyof Partial<Incomplete>) {
	switch (key) { case "C": break; }
}

function requiredKey(key: keyof Required<Incomplete>) {
	switch (key) { case "C": break; }
}

interface Parent { C: number }
interface Child extends Parent { A: number }
function inheritedKey(key: keyof Readonly<Child>) {
	switch (key) { case "C": break; }
}

function intersectedKey(key: keyof (Child & { B: number })) {
	switch (key) { case "C": break; }
}

type Indexed = { [key: string]: number; A: number };
function pickedIndexedKey(key: keyof Pick<Indexed, "A" | "C">) {
	switch (key) { case "C": break; }
}
