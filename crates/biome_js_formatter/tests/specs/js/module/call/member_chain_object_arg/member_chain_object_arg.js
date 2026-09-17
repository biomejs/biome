// https://github.com/biomejs/biome/issues/10531
// Formatter must be idempotent on member chains with object-literal args.

// Drizzle-ORM reproduction: method chain where final call arg is an object literal exceeding lineWidth.
// Already-formatted (stable) form:
import { integer, pgTable } from "drizzle-orm/pg-core";

const example = pgTable("example", {
	id: integer()
		.primaryKey()
		.generatedByDefaultAsIdentity({
			name: "example_id_seq",
			startWith: 1,
			increment: 1,
			minValue: 1,
			maxValue: 2147483647,
			cache: 1,
		}),
});

// Unformatted (inline) form — should produce the same output as above:
const example2 = pgTable("example", {
	id: integer().primaryKey().generatedByDefaultAsIdentity({ name: "example_id_seq", startWith: 1, increment: 1, minValue: 1, maxValue: 2147483647, cache: 1 }),
});

// Fastify-style reproduction: reply.code().send({...})
const handler = async (request, reply) => {
	reply.code(409).send({
		error: "Conflict",
		message: "The resource already exists with a different configuration that cannot be reconciled automatically",
	});
};

// Unformatted fastify-style:
const handler2 = async (request, reply) => {
	reply.code(409).send({ error: "Conflict", message: "The resource already exists with a different configuration that cannot be reconciled automatically" });
};

// Edge case: chain with multiple object args
const result = client.query("SELECT * FROM users").options({ timeout: 5000, retries: 3 }).transform({ format: "json", pretty: true, includeMetadata: true, nestedSerialization: true });
