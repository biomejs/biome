/* should not generate diagnostics */
// Valid: using other waitUntil options
page.waitForLoadState();

page.waitForLoadState("load");

page.waitForLoadState("domcontentloaded");

page.waitForURL(url, { waitUntil: "load" });

page.goto(url, { waitUntil: "domcontentloaded" });

page.reload({ waitUntil: "commit" });
