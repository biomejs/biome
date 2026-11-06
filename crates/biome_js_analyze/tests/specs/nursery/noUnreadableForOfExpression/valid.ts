/* should not generate diagnostics */

for (const item of items as string[]) {}
for (const item of (items satisfies Iterable<string>)) {}
for (const item of items!) {}
for (const item of <string[]>items) {}
for (const item of getItems() as string[]) {}
for (const item of items.map(callback) as string[]) {}
for (const item of items.filter(callback) as string[]) {}
for (const item of (object as Container).items) {}
for (const pair of Iterator.zip([names, scores] as const)) {}
for (const pair of Iterator.zip([names, scores] satisfies Iterable<unknown>[])) {}
for (const pair of Iterator.zip(([names, scores])!)) {}
for (const pair of Iterator.zip([names as string[], scores!])) {}
for (const item of getItems(createList<string>)) {}

function annotatedVariable() {
	const iterator: string[] = items.values();
	for (const item of iterator.map(callback)) {}
}
