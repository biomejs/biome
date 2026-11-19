// should generate diagnostics

import "./button.css";

// 1. String literal in className
function Button() {
	return <button className="btn-undefined">Invalid class</button>;
}

// 2. Variable with string literal
function Button2() {
	let cls = "btn-undefined";
	return <button className={cls}>Invalid class</button>;
}

// 3. Variable with call expression (single string arg)
function Button3() {
	let cls = clx("btn-undefined");
	return <button className={cls}>Invalid class</button>;
}

// 4. Variable with call expression (string + object args)
function Button4() {
	let cls = clx("btn-undefined", {"btn-invalid": true});
	return <button className={cls}>Invalid class</button>;
}

// 5. Inline call expression in className
function Button5() {
	return <button className={clsx("btn-undefined", "also-missing")}>Invalid class</button>;
}

// 6. Call expression with object — unquoted identifier keys
function Button6() {
	return <button className={clsx({ undeclaredA: true, undeclaredB: false })}>Invalid class</button>;
}

// 7. Call expression with array arg
function Button7() {
	return <button className={clsx(["btn-undefined", "nope"])}>Invalid class</button>;
}

// 8. Nested arrays (clsx flattens them)
function Button8() {
	return <button className={clsx(["btn-undefined", ["deeply-missing"]])}>Invalid class</button>;
}

// 9. Mixed: strings, objects, arrays (quoted and unquoted keys)
function Button9() {
	return <button className={clsx("btn-undefined", ["arr-missing"], { objMissing: true, "obj-missing-too": false })}>Invalid class</button>;
}

// 10. Variable with object expression (shorthand property)
function Button10() {
	let undeclared = "whatever";
	return <button className={clsx({ undeclared })}>Invalid class</button>;
}

// 11. class attribute (SolidJS style)
function Button11() {
	return <button class="btn-undefined">Invalid class</button>;
}
