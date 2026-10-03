/* should generate diagnostics */
// Tests that comment trivia is preserved in code actions

// Leading comment before a simple replacement
document.getElementById("foo");

// Trailing comment after the call should be preserved
document.getElementsByClassName("foo bar"); // trailing comment

/* Block comment before getElementsByName */
document.getElementsByName("email");

// Comments around the receiver should be preserved
window /* receiver comment */.document.getElementById("foo");
globalThis.document /* member-chain comment */.getElementsByClassName("foo");

// Comments around the argument should be preserved when rewriting selectors
document.getElementById(
	/* before id */ "foo" /* after id */,
);
document.getElementsByClassName(
	/* before classes */ "foo bar" /* after classes */,
);
document.getElementsByName(
	/* before name */ "email" /* after name */,
);

// Template arguments with surrounding comments
document.getElementById(
	/* before template */ `foo` /* after template */,
);
document.getElementsByClassName(
	/* before template classes */ `foo bar` /* after template classes */,
);

// Null should still be fixable without losing comments
document.getElementById(
	/* before null */ null /* after null */,
);

// Nested call chains should preserve nearby comments too
for (const div of document.body /* body comment */.getElementById("id").getElementsByClassName("class")) {
	console.log(div /* div comment */.getElementsByTagName("div"));
}

// Shadowed names are still fixable and should keep comments
function shadowed(document) {
	// inside function
	document.getElementById("foo");
}

const document = customApi;
document.getElementById("foo"); // trailing on shadowed document
