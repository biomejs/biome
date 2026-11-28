/* should generate diagnostics */

function parameterShorthand({ foo = "" }: { foo: string }) {
	return foo;
}

function parameterRenamed({ foo: bar = "" }: { foo: string }) {
	return bar;
}

const arrowParameter = ({ foo = 0 }: { foo: number }) => foo;

const functionExpressionParameter = function ({ foo = 0 }: { foo: number }) {
	return foo;
};

export default function exportedParameter({ foo = 0 }: { foo: number }) {
	return foo;
}

declare const nestedTuple: [[string], { value: string }];
const [[inner] = [""], { value: fromObject } = { value: "" }] = nestedTuple;

const { literal = "" } = { literal: "bar" };

const { a: renamed = "" } = { a: "bar" };

declare const declared: { value: number };
const { value = 1 } = declared;

declare const pair: [string, number];
const [label = "", count = 0] = pair;

declare const withHole: [string, number];
const [, second = 0] = withHole;

declare function takesCallback(callback: (value: string) => void): void;
takesCallback((value = "") => {});
takesCallback(function (value = "") {});
takesCallback(((value = "") => {}));
takesCallback((value: string = "") => {});

declare function takesSecond(callback: (first: string, second: number) => void): void;
takesSecond((first, second = 0) => {});

declare class Listener {
	constructor(callback: (value: string) => void);
}
new Listener((value = "") => {});

function parameterWithComment({ foo /* keep */ = "" }: { foo: string }) {
	return foo;
}

declare function takesObject(callback: (value: { name: string }) => void): void;
takesObject(({ name } = { name: "" }) => {});

function multiline({
	foo = "",
	bar = 0, // trailing comment
}: { foo: string; bar: number }) {
	return foo + bar;
}

declare const mixed: { known: string; [key: string]: string };
const { known = "" } = mixed;

const { "quoted": fromQuoted = 0 } = { quoted: 1 };

declare const commented: { primary: string; secondary: string };
const {
	primary // primary value
		= "",
	secondary,
} = commented;
