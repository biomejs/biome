/* should generate diagnostics */
import imported from "foo";
let blockChecked;
let commentChecked;

typeof a.b === "undefined";
typeof a.b !== "undefined";
typeof a.b == "undefined";
typeof a.b != 'undefined';
typeof /* block comment */ blockChecked === "undefined";
typeof // line comment
	commentChecked === "undefined";

let foo;
typeof foo === "undefined";

const bar = 1;
typeof bar === "undefined";

var baz;
typeof baz === "undefined";

for (const item of list) typeof item === "undefined";

function outer(param, { nested }, [first]) {
	typeof param === "undefined";
	typeof nested === "undefined";
	typeof first === "undefined";
}

function named() {
	typeof named === "undefined";
}

typeof imported.value === "undefined";

function parse(value) {
	switch (typeof value === 'undefined') {}
}

function keepReturnComment(value) {
	return typeof // comment
		value === 'undefined';
}

foo
typeof [] === "undefined";
