// should not generate diagnostics

let hey = false;

function test() {
	hey = "string"
}

if (hey) {}

declare function maybeString(): string | undefined;
function reassignedDefault(value: string | undefined = "a") {
	value = maybeString();
	return value ?? "b";
}
