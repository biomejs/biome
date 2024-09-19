throw undefined;
throw "error";
throw 0;
throw false;
throw null;
throw {};
throw "a" + "b";
const a = "";
throw a + "b";
let foo;
throw (foo = "error");
throw (new Error(), 1, 2, 3);
throw "literal" && "not an Error";
throw "literal" || new Error();
throw new Error() && "literal";
throw "literal" ?? new Error();
throw foo ? "not an Error" : "literal";
throw foo ? new Error() : "literal";
throw foo ? "literal" : new Error();
throw `${foo}`;

// False positives while valid, not a good practice.
throw "literal" && new Error();
throw new Error() || "literal";