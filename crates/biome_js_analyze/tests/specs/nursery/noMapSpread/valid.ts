/* should not generate diagnostics */
class Store {
	items: Item[] = [];
	copies() {
		return (this.items as Item[]).map((x) => ({ ...x }));
	}
	moreCopies() {
		return this.items!.map((x) => ({ ...x }));
	}
}
const unparenthesized = (a: Item[]) => a.map((x) => ({ ...x }));
const arrow = a => a.map((x) => ({ ...x }));
