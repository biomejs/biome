/* should not generate diagnostics */
fetch(url, { method: "POST", body });
new Request(url, { method: "POST", body });
fetch(url, {});
new Request(url, {});
fetch(url);
new Request(url);
fetch(url, { method: "UNKNOWN", body });
new Request(url, { method: "UNKNOWN", body });
fetch(url, { body: undefined });
new Request(url, { body: undefined });
fetch(url, { body: null });
new Request(url, { body: null });
fetch(url, { body: void 0 });
new Request(url, { method: "GET", body: void 0 });
fetch(url, { ...options, body });
new Request(url, { ...options, body });
new fetch(url, { body });
Request(url, { body });
not_fetch(url, { body });
new not_Request(url, { body });
fetch({ body }, url);
new Request({ body }, url);
fetch(url, { [body]: "foo=bar" });
new Request(url, { [body]: "foo=bar" });
fetch(url, { body: "foo=bar", body: undefined });
new Request(url, { body: "foo=bar", body: undefined });
fetch(url, { method: "HEAD", body: "foo=bar", method: "post" });
new Request(url, { method: "HEAD", body: "foo=bar", method: "post" });
fetch(url, { "method": "POST", body: "x" });
fetch(url, { ["method"]: "POST", body: "x" });
fetch(url, { body: "x", [`method`]: "POST" });

// A spread after `method` can replace it.
fetch(url, { method: "GET", ...{ method: "POST" }, body: "x" });
fetch(url, { method: "GET", ...options, body: "x" });

// A spread after `body` can replace it.
fetch(url, { body: "x", ...options });
fetch(url, { body: "x", ...options, method: "GET" });

// The method isn't known without running the code.
fetch(url, { method: getMethod(), body });
fetch(url, { method: condition ? "GET" : "POST", body });
let mutableMethod = "GET";
mutableMethod = "POST";
fetch(url, { method: mutableMethod, body: "x" });
const alias = condition;
var condition = true;
fetch(url, { method: alias ? "GET" : "POST", body });

// A body that is set to `undefined` through a variable is no body.
const noBody = undefined;
fetch(url, { body: noBody });

// The value of a getter or a setter isn't known without running the code.
fetch(url, { get method() { return "GET"; }, body: "x" });
fetch(url, { get body() { return "x"; }, set body(value) {} });

// The first argument is spread, so the options aren't the second argument.
fetch(...args, { body: "x" });

// `fetch` and `Request` are local variables.
function local(fetch, Request) {
	fetch(url, { body: "x" });
	new Request(url, { body: "x" });
}
