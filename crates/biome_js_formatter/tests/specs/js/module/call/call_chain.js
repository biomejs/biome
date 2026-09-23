// https://github.com/biomejs/biome/issues/1039
s(/🚀🚀/).s().s();

// https://github.com/biomejs/biome/issues/10531
// The formatter must be idempotent on member chains whose final call argument
// is an object literal. The inline form below and the already-formatted form
// must both format to the same output: the chain stays inline and only the
// object argument breaks.
const example = pgTable("example", {
	id: integer().primaryKey().generatedByDefaultAsIdentity({ name: "example_id_seq", startWith: 1, increment: 1, minValue: 1, maxValue: 2147483647, cache: 1 }),
});

const examplePreformatted = pgTable("example", {
	id: integer().primaryKey().generatedByDefaultAsIdentity({
		name: "example_id_seq",
		startWith: 1,
		increment: 1,
		minValue: 1,
		maxValue: 2147483647,
		cache: 1,
	}),
});

// A chain whose final object argument is small enough to fit on its own line
// keeps the fully-expanded layout (the object does not hug).
const d3Like = d3.select("body").append("circle").at({ width: 30, fill: "#f0f" }).st({ fontWeight: 600 });
