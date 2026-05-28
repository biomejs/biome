/* should not generate diagnostics */
typeof a.b;
typeof a.b > "undefined";
a.b === "undefined";
void a.b === "undefined";
+a.b === "undefined";
++a.b === "undefined";
a.b++ === "undefined";
foo === undefined;
typeof a.b === "string";
typeof foo === "undefined";
foo = 2;
typeof foo === "undefined";

function parse() {
	switch (typeof value === 'undefined') {}
}

"undefined" === typeof a.b;
const UNDEFINED = "undefined";
typeof a.b === UNDEFINED;
typeof a.b === `undefined`;
