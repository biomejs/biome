import {
	Cause,
	Context,
	Data,
	Duration,
	Effect,
	Either,
	Exit,
	Layer,
	Match,
	Option,
	Ref,
	Schedule,
	pipe,
} from "./vendor/effect";
import { type, type Type } from "./vendor/arktype";
import {
	EventEnvelope,
	Order,
	OrderDraft,
	OrderPatch,
	OrderPage,
	Product,
	RuntimeConfig,
	SearchQuery,
	eventLabel,
	type Money,
	type Order as OrderShape,
	type OrderDraft as OrderDraftShape,
	type OrderEvent,
	type OrderSummary,
	type Product as ProductShape,
	type RuntimeConfig as RuntimeConfigShape,
	type SearchQuery as SearchQueryShape,
} from "./schema";

export class ValidationError extends Data.TaggedError("ValidationError")<{
	readonly schema: string;
	readonly summary: string;
	readonly paths: ReadonlyArray<string>;
}> {}

export class NotFound extends Data.TaggedError("NotFound")<{
	readonly entity: "order" | "product" | "customer";
	readonly id: string;
}> {}

export class Conflict extends Data.TaggedError("Conflict")<{
	readonly id: string;
	readonly expected: number;
	readonly actual: number;
}> {}

export class PaymentFailed extends Data.TaggedError("PaymentFailed")<{
	readonly reason: "declined" | "timeout" | "provider";
	readonly retryable: boolean;
}> {}

export class DatabaseError extends Data.TaggedError("DatabaseError")<{
	readonly query: string;
	readonly cause: unknown;
}> {}

export class OutOfStock extends Data.TaggedError("OutOfStock")<{
	readonly sku: string;
	readonly requested: number;
	readonly available: number;
}> {}

export const decode =
	<S extends Type<any, any>>(name: string, schema: S) =>
	(input: unknown): Effect.Effect<S["infer"], ValidationError> => {
		const out = schema(input);
		return out instanceof type.errors
			? Effect.fail(
					new ValidationError({
						schema: name,
						summary: out.summary,
						paths: out.map((error) => error.propString),
					}),
				)
			: Effect.succeed(out as S["infer"]);
	};

const decodeDraft = decode("OrderDraft", OrderDraft);
const decodeOrder = decode("Order", Order);
const decodePatch = decode("OrderPatch", OrderPatch);
const decodeSearch = decode("SearchQuery", SearchQuery);
const decodeEnvelope = decode("EventEnvelope", EventEnvelope);
const decodeProduct = decode("Product", Product);
const decodePage = decode("OrderPage", OrderPage);

export class AppConfig extends Context.Tag("AppConfig")<
	AppConfig,
	RuntimeConfigShape
>() {}

export class Clock extends Context.Tag("Clock")<
	Clock,
	{ readonly now: Effect.Effect<Date> }
>() {}

export class Database extends Context.Tag("Database")<
	Database,
	{
		readonly query: <A, E>(
			sql: string,
			params: ReadonlyArray<unknown>,
			decoder: (row: unknown) => Effect.Effect<A, E>,
		) => Effect.Effect<ReadonlyArray<A>, DatabaseError | E>;
		readonly transaction: <A, E, R>(
			body: Effect.Effect<A, E, R>,
		) => Effect.Effect<A, E | DatabaseError, R>;
	}
>() {}

export class Payments extends Context.Tag("Payments")<
	Payments,
	{
		readonly authorize: (
			order: OrderShape,
		) => Effect.Effect<
			{ authorizationId: string; amount: Money },
			PaymentFailed
		>;
		readonly refund: (
			authorizationId: string,
			amount: Money,
		) => Effect.Effect<void, PaymentFailed>;
	}
>() {}

export class Inventory extends Context.Tag("Inventory")<
	Inventory,
	{
		readonly reserve: (
			items: OrderDraftShape["items"],
		) => Effect.Effect<Map<string, number>, OutOfStock | DatabaseError>;
		readonly release: (
			reservations: Map<string, number>,
		) => Effect.Effect<void>;
		readonly product: (
			sku: string,
		) => Effect.Effect<
			Option.Option<ProductShape>,
			DatabaseError | ValidationError
		>;
	}
>() {}

export class EventLog extends Context.Tag("EventLog")<
	EventLog,
	{
		readonly append: (
			orderId: string,
			events: ReadonlyArray<OrderEvent>,
		) => Effect.Effect<number, DatabaseError | Conflict>;
		readonly read: (
			orderId: string,
			from: number,
		) => Effect.Effect<
			typeof EventEnvelope.infer,
			DatabaseError | ValidationError
		>;
	}
>() {}

export class Metrics extends Effect.Service<Metrics>()("Metrics", {
	effect: Effect.gen(function* () {
		const counters = yield* Ref.make(new Map<string, number>());
		const histograms = yield* Ref.make(
			new Map<string, ReadonlyArray<number>>(),
		);
		const increment = (name: string, by = 1) =>
			Ref.update(counters, (map) =>
				new Map(map).set(name, (map.get(name) ?? 0) + by),
			);
		const observe = (name: string, value: number) =>
			Ref.update(histograms, (map) =>
				new Map(map).set(name, [...(map.get(name) ?? []), value]),
			);
		const snapshot = Effect.all({
			counters: Ref.get(counters),
			histograms: Ref.get(histograms),
		});
		return { increment, observe, snapshot } as const;
	}),
}) {}

export const ClockLive = Layer.succeed(Clock, {
	now: Effect.sync(() => new Date()),
});

export const AppConfigLive = Layer.effect(
	AppConfig,
	Effect.gen(function* () {
		const raw = yield* Effect.sync(() => JSON.parse("{}") as unknown);
		return yield* decode("RuntimeConfig", RuntimeConfig)(raw);
	}),
);

export const DatabaseLive = Layer.scoped(
	Database,
	Effect.gen(function* () {
		const config = yield* AppConfig;
		const pool = yield* Effect.acquireRelease(
			Effect.sync(() => ({
				url: config.database.url,
				size: config.database.poolSize,
				open: true,
			})),
			(pool) => Effect.sync(() => void (pool.open = false)),
		);
		const query = <A, E>(
			sql: string,
			params: ReadonlyArray<unknown>,
			decoder: (row: unknown) => Effect.Effect<A, E>,
		) =>
			pipe(
				Effect.tryPromise({
					try: () => Promise.resolve([] as unknown[]),
					catch: (cause) => new DatabaseError({ query: sql, cause }),
				}),
				Effect.flatMap((rows) => Effect.forEach(rows, decoder)),
				Effect.timeoutFail({
					duration: Duration.seconds(pool.size > 10 ? 5 : 15),
					onTimeout: () => new DatabaseError({ query: sql, cause: "timeout" }),
				}),
				Effect.withSpan("db.query", {
					attributes: { sql, params: params.length },
				}),
			);
		const transaction = <A, E, R>(body: Effect.Effect<A, E, R>) =>
			pipe(
				query("BEGIN", [], Effect.succeed),
				Effect.zipRight(body),
				Effect.tap(() => query("COMMIT", [], Effect.succeed)),
				Effect.tapErrorCause(() =>
					Effect.ignore(query("ROLLBACK", [], Effect.succeed)),
				),
			);
		return { query, transaction };
	}),
);

export const PaymentsLive = Layer.effect(
	Payments,
	Effect.gen(function* () {
		const config = yield* AppConfig;
		const metrics = yield* Metrics;
		const policy = pipe(
			Schedule.exponential(Duration.millis(100), 2),
			Schedule.jittered,
			Schedule.intersect(Schedule.recurs(4)),
			Schedule.whileInput((error: PaymentFailed) => error.retryable),
		);
		const call = <A>(name: string, attempt: () => Promise<A>) =>
			pipe(
				Effect.tryPromise({
					try: attempt,
					catch: () =>
						new PaymentFailed({ reason: "provider", retryable: true }),
				}),
				Effect.timeoutFail({
					duration: Duration.millis(config.payments.timeoutMs),
					onTimeout: () =>
						new PaymentFailed({ reason: "timeout", retryable: true }),
				}),
				Effect.retry(policy),
				Effect.tap(() => metrics.increment(`payments.${name}.ok`)),
				Effect.tapError(() => metrics.increment(`payments.${name}.failed`)),
			);
		return {
			authorize: (order: OrderShape) =>
				call("authorize", async () => ({
					authorizationId: `${config.payments.provider}_${order.id}`,
					amount: order.totals.grand,
				})),
			refund: (authorizationId: string, amount: Money) =>
				call("refund", async () => void [authorizationId, amount]),
		};
	}),
);

export const InventoryLive = Layer.effect(
	Inventory,
	Effect.gen(function* () {
		const db = yield* Database;
		const reserved = yield* Ref.make(new Map<string, number>());
		const product = (sku: string) =>
			pipe(
				db.query("select * from products where sku = $1", [sku], decodeProduct),
				Effect.map((rows) => Option.fromNullable(rows[0])),
			);
		const reserve = (items: OrderDraftShape["items"]) =>
			db.transaction(
				Effect.gen(function* () {
					const levels = yield* Effect.forEach(
						items,
						(item) =>
							pipe(
								product(item.sku),
								Effect.map(
									Option.match({
										onNone: () => 0,
										onSome: (p) =>
											p.variants.find((variant) => variant.sku === item.sku)
												?.stock ?? 0,
									}),
								),
								Effect.orElseSucceed(() => 0),
								Effect.map((available) => ({ item, available })),
							),
						{ concurrency: 8 },
					);
					for (const { item, available } of levels) {
						if (available < item.quantity) {
							return yield* new OutOfStock({
								sku: item.sku,
								requested: item.quantity,
								available,
							});
						}
					}
					const reservation = new Map(
						levels.map(({ item }) => [item.sku, item.quantity] as const),
					);
					yield* Ref.update(
						reserved,
						(current) => new Map([...current, ...reservation]),
					);
					return reservation;
				}),
			);
		const release = (reservation: Map<string, number>) =>
			Ref.update(reserved, (current) => {
				const next = new Map(current);
				for (const sku of reservation.keys()) next.delete(sku);
				return next;
			});
		return { reserve, release, product };
	}),
);

export const EventLogLive = Layer.effect(
	EventLog,
	Effect.gen(function* () {
		const db = yield* Database;
		const clock = yield* Clock;
		return {
			append: (orderId: string, events: ReadonlyArray<OrderEvent>) =>
				Effect.gen(function* () {
					const now = yield* clock.now;
					const [head] = yield* db.query(
						"select max(version) as version from events where order_id = $1",
						[orderId],
						decode("Version", type({ version: "number.integer" })),
					);
					const expected = events[0]?.version ?? 1;
					if (head && head.version + 1 !== expected) {
						return yield* new Conflict({
							id: orderId,
							expected,
							actual: head.version + 1,
						});
					}
					yield* db.query(
						"insert into events values ($1, $2, $3)",
						[orderId, now.toISOString(), JSON.stringify(events)],
						Effect.succeed,
					);
					return expected + events.length - 1;
				}).pipe(
					Effect.catchTag("ValidationError", (error) =>
						Effect.fail(
							new DatabaseError({ query: "events.head", cause: error }),
						),
					),
				),
			read: (orderId: string, from: number) =>
				pipe(
					db.query(
						"select * from events where order_id = $1 and version >= $2",
						[orderId, from],
						Effect.succeed,
					),
					Effect.flatMap((rows) =>
						decodeEnvelope({ stream: orderId, sequence: from, events: rows }),
					),
				),
		};
	}),
);

export const priceLine = (item: OrderDraftShape["items"][number]): Money => {
	const gross = item.unitPrice.amount * item.quantity;
	const discount = (item.discounts ?? []).reduce(
		(total, d) =>
			total +
			(d.kind === "percent" ? Math.round((gross * d.value) / 100) : d.value),
		0,
	);
	return {
		amount: Math.max(0, gross - discount),
		currency: item.unitPrice.currency,
	};
};

const shippingRate = Match.type<OrderDraftShape["shippingMethod"]>().pipe(
	Match.when("pickup", () => 0),
	Match.when("ground", () => 599),
	Match.when("express", () => 1499),
	Match.when("overnight", () => 3499),
	Match.exhaustive,
);

const taxRate = Match.type<
	OrderDraftShape["customer"]["addresses"]["billing"]
>().pipe(
	Match.when({ country: "US", region: "CA" }, () => 0.0725),
	Match.when({ country: "US" }, () => 0.05),
	Match.when({ country: Match.is("GB", "DE", "FR") }, () => 0.2),
	Match.when({ country: "JP" }, () => 0.1),
	Match.orElse(() => 0),
);

export const quote = (draft: OrderDraftShape) =>
	Effect.gen(function* () {
		const lines = draft.items.map(priceLine);
		const currency = lines[0]?.currency ?? "USD";
		const mixed = lines.find((line) => line.currency !== currency);
		if (mixed) {
			return yield* new ValidationError({
				schema: "OrderDraft",
				summary: `mixed currencies ${currency} and ${mixed.currency}`,
				paths: ["items"],
			});
		}
		const subtotal = lines.reduce((total, line) => total + line.amount, 0);
		const tierDiscount = pipe(
			Match.value(draft.customer.tier),
			Match.when("platinum", () => 0.1),
			Match.when("gold", () => 0.05),
			Match.orElse(() => 0),
		);
		const discount = Math.round(subtotal * tierDiscount);
		const shipping = shippingRate(draft.shippingMethod);
		const tax = Math.round(
			(subtotal - discount) * taxRate(draft.customer.addresses.billing),
		);
		const money = (amount: number): Money => ({ amount, currency });
		return {
			subtotal: money(subtotal),
			tax: money(tax),
			shipping: money(shipping),
			discount: money(discount),
			grand: money(subtotal - discount + shipping + tax),
		} satisfies OrderShape["totals"];
	});

export const applyEvent = (order: OrderShape, event: OrderEvent): OrderShape =>
	Match.value(event).pipe(
		Match.discriminatorsExhaustive("type")({
			"order.placed": (e) => ({
				...order,
				...e.payload,
				status: "pending" as const,
			}),
			"payment.authorized": () => ({ ...order, status: "authorized" as const }),
			"payment.declined": (e) =>
				e.payload.retryable
					? order
					: { ...order, status: "cancelled" as const },
			"items.picked": () => ({ ...order, status: "picking" as const }),
			"shipment.created": () => ({ ...order, status: "shipped" as const }),
			"order.cancelled": (e) => ({
				...order,
				status: "cancelled" as const,
				notes: [order.notes, e.payload.reason].filter(Boolean).join("\n"),
			}),
		}),
		(next) => ({ ...next, revision: event.version }),
	);

export const placeOrder = Effect.fn("placeOrder")(function* (input: unknown) {
	const draft = yield* decodeDraft(input);
	const inventory = yield* Inventory;
	const payments = yield* Payments;
	const log = yield* EventLog;
	const clock = yield* Clock;
	const metrics = yield* Metrics;

	const totals = yield* quote(draft);
	const now = yield* clock.now;
	const id = crypto.randomUUID();
	const order = yield* decodeOrder({
		...draft,
		id,
		status: "pending",
		totals,
		revision: 0,
		requestedAt: draft.requestedAt.toISOString(),
		createdAt: draft.customer.createdAt.toISOString(),
		customer: {
			...draft.customer,
			createdAt: draft.customer.createdAt.toISOString(),
			flags: [...draft.customer.flags],
		},
	});

	const reservation = yield* Effect.acquireRelease(
		inventory.reserve(draft.items),
		(held, exit) =>
			Exit.isSuccess(exit) ? Effect.void : inventory.release(held),
	);

	const authorization = yield* pipe(
		payments.authorize(order),
		Effect.either,
		Effect.flatMap(
			Either.match({
				onLeft: (error) =>
					pipe(
						log.append(id, [
							{
								eventId: crypto.randomUUID(),
								orderId: id,
								at: now,
								version: 1,
								type: "payment.declined",
								payload: { reason: "unknown", retryable: error.retryable },
							},
						]),
						Effect.zipRight(Effect.fail(error)),
					),
				onRight: Effect.succeed,
			}),
		),
	);

	const placed: OrderEvent = {
		eventId: crypto.randomUUID(),
		orderId: id,
		at: now,
		version: 1,
		type: "order.placed",
		payload: draft,
	};
	const authorized: OrderEvent = {
		eventId: crypto.randomUUID(),
		orderId: id,
		at: now,
		version: 2,
		type: "payment.authorized",
		payload: {
			authorizationId: authorization.authorizationId,
			amount: authorization.amount,
		},
	};
	const revision = yield* log.append(id, [placed, authorized]);
	yield* metrics.increment("orders.placed");
	yield* metrics.observe("orders.value", totals.grand.amount);
	yield* Effect.logInfo(eventLabel(placed), eventLabel(authorized));

	return {
		order: [placed, authorized].reduce(applyEvent, order),
		revision,
		reservation: Object.fromEntries(reservation),
	};
}, Effect.scoped);

export const patchOrder = (id: string, input: unknown) =>
	Effect.gen(function* () {
		const patch = yield* decodePatch(input);
		const log = yield* EventLog;
		const envelope = yield* log.read(id, 0);
		if (envelope.events.length === 0) {
			return yield* new NotFound({ entity: "order", id });
		}
		const [first, ...rest] = envelope.events;
		if (first.type !== "order.placed") {
			return yield* new ValidationError({
				schema: "EventEnvelope",
				summary: `stream ${id} does not start with order.placed`,
				paths: ["events", "0", "type"],
			});
		}
		const seed = yield* decodeOrder({
			...first.payload,
			id,
			status: "pending",
			revision: 0,
			totals: yield* quote(first.payload),
		});
		const current = rest.reduce(applyEvent, applyEvent(seed, first));
		if (current.revision !== patch.revision) {
			return yield* new Conflict({
				id,
				expected: patch.revision,
				actual: current.revision,
			});
		}
		return { ...current, ...patch, revision: current.revision + 1 };
	});

export const searchOrders = (raw: Record<string, string | string[]>) =>
	Effect.gen(function* () {
		const query: SearchQueryShape = yield* decodeSearch(raw);
		const db = yield* Database;
		const candidates: Array<readonly [string, unknown] | null> = [
			query.q ? ["customer_email ilike $?", `%${query.q}%`] : null,
			query.status?.length ? ["status = any($?)", query.status] : null,
			query.channel?.length ? ["channel = any($?)", query.channel] : null,
			query.minTotal === undefined ? null : ["total >= $?", query.minTotal],
			query.maxTotal === undefined ? null : ["total <= $?", query.maxTotal],
			query.placedAfter
				? ["placed_at >= $?", query.placedAfter.toISOString()]
				: null,
		];
		const clauses = candidates.filter((clause) => clause !== null);
		const rows = yield* db.query(
			`select * from orders where ${clauses.map(([sql]) => sql).join(" and ")} order by ${query.sort} ${query.direction}`,
			clauses.map(([, value]) => value),
			decodeOrder,
		);
		const summaries: OrderSummary[] = rows.map(
			({ customer, payment, items, ...order }) => ({
				...order,
				itemCount: items.reduce((total, item) => total + item.quantity, 0),
				customerEmail: customer.email,
			}),
		);
		return yield* decodePage({
			items: summaries.slice(
				query.page * query.size,
				(query.page + 1) * query.size,
			),
			total: summaries.length,
			page: query.page,
			size: query.size,
			next:
				summaries.length > (query.page + 1) * query.size
					? String(query.page + 1)
					: null,
		});
	});

export const replay = (orderIds: ReadonlyArray<string>) =>
	Effect.gen(function* () {
		const log = yield* EventLog;
		const results = yield* Effect.forEach(
			orderIds,
			(id) =>
				pipe(
					log.read(id, 0),
					Effect.map((envelope) => envelope.events.map(eventLabel)),
					Effect.either,
				),
			{ concurrency: "unbounded" },
		);
		const [failures, successes] = results.reduce<[string[], string[][]]>(
			([left, right], result) =>
				Either.isLeft(result)
					? [[...left, result.left._tag], right]
					: [left, [...right, result.right]],
			[[], []],
		);
		return { failures, labels: successes.flat() };
	});

export const handle = (
	route: "place" | "patch" | "search" | "replay",
	payload: unknown,
) =>
	pipe(
		Match.value(route),
		Match.when("place", () => placeOrder(payload)),
		Match.when("patch", () =>
			patchOrder(String((payload as { id?: unknown }).id), payload),
		),
		Match.when("search", () => searchOrders(payload as Record<string, string>)),
		Match.when("replay", () => replay(payload as string[])),
		Match.exhaustive,
	).pipe(
		Effect.map((body) => ({ status: 200 as const, body })),
		Effect.catchTags({
			ValidationError: (error) =>
				Effect.succeed({
					status: 422 as const,
					body: { error: error.summary, paths: error.paths },
				}),
			NotFound: (error) =>
				Effect.succeed({
					status: 404 as const,
					body: { error: `${error.entity} ${error.id}` },
				}),
			Conflict: (error) =>
				Effect.succeed({
					status: 409 as const,
					body: {
						error: "revision mismatch",
						expected: error.expected,
						actual: error.actual,
					},
				}),
			OutOfStock: (error) =>
				Effect.succeed({
					status: 409 as const,
					body: { error: `sku ${error.sku} unavailable` },
				}),
			PaymentFailed: (error) =>
				Effect.succeed({ status: 402 as const, body: { error: error.reason } }),
		}),
		Effect.catchAllCause((cause) =>
			Effect.succeed({
				status: 500 as const,
				body: { error: Cause.pretty(cause) },
			}),
		),
	);

export const MainLive = pipe(
	Layer.mergeAll(InventoryLive, PaymentsLive, EventLogLive),
	Layer.provideMerge(DatabaseLive),
	Layer.provideMerge(Metrics.Default),
	Layer.provideMerge(ClockLive),
	Layer.provideMerge(AppConfigLive),
);

export const program = Effect.gen(function* () {
	const responses = yield* Effect.all(
		{
			placed: handle("place", {}),
			patched: handle("patch", {
				id: "order-1",
				status: "packed",
				revision: 2,
			}),
			search: handle("search", {
				q: "ada",
				status: ["pending", "shipped"],
				size: "10",
			}),
			replay: handle("replay", ["order-1", "order-2"]),
		},
		{ concurrency: 2 },
	);
	const stats = yield* Metrics.pipe(
		Effect.flatMap((metrics) => metrics.snapshot),
	);
	return { responses, stats };
}).pipe(Effect.provide(MainLive));

export const run = () => Effect.runPromiseExit(program);
