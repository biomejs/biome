/* should generate diagnostics */
function foo() {
	new Error();
}

const bar = () => {
	new Error("message");
};

new TypeError("message")

new RangeError;

(new Error("parenthesized"));

new lib.CustomError();

new (Error)();

if (condition) new Error("in if");

function withComment() {
	// leading comment
	new ValidationError("message"); // trailing comment
}
