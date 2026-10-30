/* should generate diagnostics */
fetch(url, { method: "GET", body: "x" } as RequestInit);
fetch(url, { method: "GET", body: "x" } satisfies RequestInit);
fetch(url, { method: "GET", body: "x" }!);
fetch(url, <RequestInit>{ method: "GET", body: "x" });
new Request(url, { method: "GET", body: "x" } as RequestInit);
fetch(url, { method: "GET" as const, body: "x" });
const typedMethod: string = "HEAD";
fetch(url, { method: typedMethod, body: "x" });
