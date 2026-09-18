/* should not generate diagnostics */

import "./styles.css";

// 1. String literal with defined classes
function Component() {
	return <button className="btn primary">Valid classes</button>;
}

// 2. Variable with string literal
function Component2() {
	let cls = "btn";
	return <button className={cls}>Valid</button>;
}

// 3. Variable with call expression
function Component3() {
	let cls = clsx("btn", "primary");
	return <button className={cls}>Valid</button>;
}

// 4. Inline call expression with object — unquoted identifier key
function Component4() {
	return <button className={clsx("btn", { primary: true })}>Valid</button>;
}

// 4b. Inline call expression with object — quoted string key
function Component4b() {
	return <button className={clsx({ "primary": true, "container": false })}>Valid</button>;
}

// 5. Call expression with array
function Component5() {
	return <button className={clsx(["btn", "container"])}>Valid</button>;
}

// 6. Mixed: strings, objects, arrays — all defined
function Component6() {
	return <button className={clsx("btn", ["primary"], { container: true })}>Valid</button>;
}

// 7. Dynamic expressions should be silently skipped (no false positives)
function Component7() {
	let dynamic = someFunction();
	return <button className={dynamic}>Valid</button>;
}

// 8. Template literal — skipped
function Component8() {
	return <button className={`btn ${condition ? "primary" : ""}`}>Valid</button>;
}

// 9. class attribute (SolidJS style) with defined classes
function Component9() {
	return <button class="btn primary">Valid</button>;
}
