/* should generate diagnostics */
{
	if ((foo as string) === "one") {}
	if (foo === "two") {}
}

{
	if (foo! === "one") {}
	if (foo === "two") {}
}

{
	if ((foo satisfies string) === "one") {}
	if (foo === "two") {}
}

function boolean1(foo: boolean) {
	if (foo) {}
	if (foo === false) {}
}

function boolean2(foo: boolean) {
	if (!foo) {}
	if (foo === true) {}
}

function boolean3(foo: boolean) {
	if (Boolean(foo)) {}
	if (foo === false) {}
}

function boolean4(foo: boolean) {
	if (foo === false) {}
	if (Boolean(foo)) {}
}

function boolean5(foo: true | false) {
	if (!!foo) {}
	if (!foo) {}
}

function boolean6(foo: boolean) {
	if (foo === false) {}

	if (foo === true) {}
	else if (Boolean(foo)) {}
}

class Options {
	constructor(private enabled: boolean) {
		if (enabled) {}
		if (enabled === false) {}
	}
}

{
	let flag: boolean = read();
	if (flag) {}
	if (!flag) {}
}
