/* should not generate diagnostics */
page;
page.locator("button");
frame.locator("button");
foo.locator("button");

page.locator("button").click();
frame.locator("button").click();
foo.locator("button").click();
foo.click();

page.getByRole("button", { name: "Submit" });
page.getByRole("listitem").filter({ hasText: "Product 2" });

// Not a method call.
first();
last();
nth(1);
page.locator("button").first;
const { first } = locator;

// The method name is not known until the code runs.
page.locator("button")[first]();
page.locator("button")[`${prefix}first`]();
page.locator("button")[0]();

// Other names.
page.locator("button").firstChild();
page.locator("button").First();
page.locator("button").nthChild(1);
page.locator("button")["lastOne"]();

class Buttons {
	#first() {}
	#last() {}
	run() {
		this.#first();
		this.#last();
	}
}

new foo.first();
foo.first`template`;
