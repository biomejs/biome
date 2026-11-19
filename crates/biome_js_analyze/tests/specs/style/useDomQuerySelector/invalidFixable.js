/* should generate diagnostics */
document.getElementById("foo");
document.getElementsByClassName("foo");
document.getElementsByClassName("foo bar");
document.getElementsByTagName("main");
document.getElementsByName("email");
document.getElementById(`foo`);
document.getElementsByClassName(`foo bar`);
document.getElementsByName(`email`);
document.getElementById(null);
window.document.getElementById("foo");
globalThis.document.getElementsByClassName("foo");
element.getElementsByClassName("foo");
customApi.getElementById("foo");

function shadowed(document) {
	document.getElementById("foo");
}

const document = customApi;
document.getElementById("foo");

for (const div of document.body.getElementById("id").getElementsByClassName("class")) {
	console.log(div.getElementsByTagName("div"));
}
