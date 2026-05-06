/* should generate diagnostics */
// Test all methods that support networkidle option
page.waitForLoadState("networkidle");

page.waitForURL("http://example.com", { waitUntil: "networkidle" });

page.goto("http://example.com", { waitUntil: "networkidle" });

page.reload({ waitUntil: "networkidle" });

page.setContent("<html></html>", { waitUntil: "networkidle" });

page.goBack({ waitUntil: "networkidle" });

page.goForward({ waitUntil: "networkidle" });
