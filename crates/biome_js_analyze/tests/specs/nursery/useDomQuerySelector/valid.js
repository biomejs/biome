/* should not generate diagnostics */
document.querySelector("#foo");
document.querySelectorAll(".foo");
document?.getElementById("foo");
document.getElementById?.("foo");
document.getElementById();
document.getElementById("foo", "bar");
document["getElementById"]("foo");
(undefined).getElementById("foo");
({}).getElementById("foo");
[1, 2, 3].getElementById("foo");
(() => {}).getElementById("foo");
`template`.getElementById("foo");
