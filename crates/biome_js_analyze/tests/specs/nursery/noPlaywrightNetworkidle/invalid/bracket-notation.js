// Test bracket notation for networkidle option
page["waitForLoadState"]("networkidle");

page[`waitForLoadState`]("networkidle");

page["waitForURL"](url, { waitUntil: "networkidle" });

page[`goto`](url, { waitUntil: "networkidle" });

page["reload"](url, { waitUntil: "networkidle" });
