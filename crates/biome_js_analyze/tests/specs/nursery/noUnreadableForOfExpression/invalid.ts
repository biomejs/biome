/* should generate diagnostics */

for (const item of getItems(createArgument(seed)) as string[]) {}
for (const item of (Iterator as typeof Iterator).from(items).map(callback)) {}
for (const item of (Iterator.from(items) as Iterator<string>).map(callback)) {}
for (const item of Iterator.from(items).take(1) as Iterator<string>) {}
for (const item of (items[index] satisfies Iterable<string>)) {}
for (const pair of Iterator.zip([names, getScores(seed)] as const)) {}
for (const pair of Iterator.zip([[names], [scores]] as const)) {}
for (const pair of Iterator.zip([getNames(seed) as string[], scores])) {}
for (const item of (getItems as Getter)()) {}
