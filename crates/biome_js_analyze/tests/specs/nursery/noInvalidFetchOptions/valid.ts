/* should not generate diagnostics */
fetch(url, { method: "POST", body: "x" } as RequestInit);
fetch(url, { method: "POST" as const, body: "x" });
declare const method: string;
fetch(url, { method, body: "x" });
