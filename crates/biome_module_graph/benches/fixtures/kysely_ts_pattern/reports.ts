import { match, isMatching, P, NonExhaustiveError } from "./vendor/ts-pattern";
import type {
	CountryCode,
	Currency,
	EventPayload,
	OrderStatus,
	PaymentPayload,
	UserMetadata,
} from "./schema";
import {
	buyerSellerPairs,
	categoryTree,
	eventFeed,
	orderDetails,
	paymentsNeedingAttention,
	placeOrder,
	revenueByBucket,
	salesByStore,
	topProducts,
	transitionOrder,
	type AttentionPayment,
	type BucketRow,
	type CategoryNode,
	type DateRange,
	type EventRow,
	type OrderDetails,
	type PairRow,
	type PlaceOrderInput,
	type SalesRow,
	type TopProduct,
} from "./queries";

// Errors surfaced by the data layer.

export class QueryTimeout extends Error {
	constructor(
		readonly query: string,
		readonly elapsedMs: number,
	) {
		super(`query ${query} timed out after ${elapsedMs}ms`);
	}
}

export class ConstraintViolation extends Error {
	constructor(
		readonly constraint:
			`${string}_pkey` | `${string}_fkey` | `${string}_check`,
		readonly table: string,
	) {
		super(`constraint ${constraint} violated on ${table}`);
	}
}

export class SerializationFailure extends Error {
	readonly code = "40001" as const;
}

export type Recovery =
	| { readonly kind: "retry"; readonly afterMs: number }
	| { readonly kind: "conflict"; readonly table: string }
	| { readonly kind: "fatal"; readonly message: string };

export function classifyError(error: unknown) {
	return match(error)
		.with(P.instanceOf(SerializationFailure), () => ({
			kind: "retry" as const,
			afterMs: 25,
		}))
		.with(
			P.instanceOf(QueryTimeout).and({ elapsedMs: P.number.lt(30_000) }),
			(timeout) => ({ kind: "retry" as const, afterMs: timeout.elapsedMs * 2 }),
		)
		.with(
			P.instanceOf(ConstraintViolation).and({
				constraint: P.string.endsWith("_pkey"),
			}),
			({ table }) => ({ kind: "conflict" as const, table }),
		)
		.with(
			P.instanceOf(ConstraintViolation),
			P.instanceOf(QueryTimeout),
			(err) => ({ kind: "fatal" as const, message: err.message }),
		)
		.with({ message: P.select(P.string) }, (message) => ({
			kind: "fatal" as const,
			message,
		}))
		.otherwise(() => ({ kind: "fatal" as const, message: String(error) }));
}

export function recoveryDelay(recovery: Recovery) {
	return match(recovery)
		.with({ kind: "retry", afterMs: P.number.lte(100) }, () => 0)
		.with({ kind: "retry" }, ({ afterMs }) => Math.min(afterMs, 5_000))
		.with({ kind: P.union("conflict", "fatal") }, () => -1)
		.exhaustive();
}

// Domain events.

export type EventAction =
	| { readonly action: "notify"; readonly channel: "ops" | "finance" | "trust" }
	| { readonly action: "reindex"; readonly productId: string }
	| { readonly action: "refund"; readonly orderId: string }
	| { readonly action: "ignore" };

export function routeEvent(payload: EventPayload): EventAction {
	return match(payload)
		.with(
			{ type: "order.placed", totalCents: P.number.gte(500_000) },
			{ type: "order.placed", currency: P.union("JPY", "CAD") },
			() => ({ action: "notify" as const, channel: "finance" as const }),
		)
		.with(
			{ type: "order.placed" },
			(event) => event.totalCents > 100_000 && event.currency === "USD",
			() => ({ action: "notify" as const, channel: "ops" as const }),
		)
		.with({ type: "order.placed" }, () => ({ action: "ignore" as const }))
		.with(
			{
				type: "order.status_changed",
				from: P.union("paid", "packed", "shipped"),
				to: "refunded",
				orderId: P.select(),
			},
			(orderId) => ({ action: "refund" as const, orderId }),
		)
		.with({ type: "order.status_changed" }, () => ({
			action: "ignore" as const,
		}))
		.with({ type: "payment.failed", retryable: false }, () => ({
			action: "notify" as const,
			channel: "ops" as const,
		}))
		.with(
			{ type: "payment.failed", code: P.string.startsWith("fraud_") },
			() => ({
				action: "notify" as const,
				channel: "trust" as const,
			}),
		)
		.with({ type: "payment.failed" }, () => ({ action: "ignore" as const }))
		.with(
			{ type: "inventory.adjusted", delta: P.number.negative() },
			{ type: "inventory.adjusted", warehouse: P.string.startsWith("EU-") },
			({ productId }) => ({ action: "reindex" as const, productId }),
		)
		.with({ type: "inventory.adjusted" }, () => ({ action: "ignore" as const }))
		.with(
			{ type: "review.flagged", reasons: P.array(P.union("abuse", "spam")) },
			() => ({ action: "notify" as const, channel: "trust" as const }),
		)
		.with({ type: "review.flagged" }, () => ({ action: "ignore" as const }))
		.exhaustive();
}

export function describeEvent(row: EventRow) {
	return match(row)
		.with(
			{
				aggregate_type: "order",
				payload: {
					type: "order.placed",
					totalCents: P.select("total"),
					currency: P.select("currency"),
				},
			},
			({ total, currency }) =>
				`placed ${formatMoney(total, currency)}` as const,
		)
		.with(
			{
				aggregate_type: "order",
				payload: {
					type: "order.status_changed",
					from: P.select("from"),
					to: P.select("to"),
				},
			},
			({ from, to }) => `${from} -> ${to}` as const,
		)
		.with(
			{
				aggregate_type: "payment",
				payload: { type: "payment.failed", code: P.select() },
			},
			(code) => `payment failed: ${code}` as const,
		)
		.with(
			{ processed_at: P.not(P.nullish), occurred_at: P.instanceOf(Date) },
			(processed) => `already processed ${processed.id}` as const,
		)
		.otherwise((other) => `unhandled ${other.payload.type}` as const);
}

export const refundRequestPattern = {
	type: "order.status_changed",
	orderId: P.string.minLength(8),
	from: P.union("paid", "packed", "shipped", "delivered"),
	to: "refunded",
} as const;

export type RefundRequest = P.infer<typeof refundRequestPattern>;

export function refundRequests(rows: ReadonlyArray<EventRow>) {
	const requests: Array<{ id: string; from: RefundRequest["from"] }> = [];
	for (const row of rows) {
		if (isMatching({ payload: refundRequestPattern }, row)) {
			requests.push({ id: row.payload.orderId, from: row.payload.from });
		}
	}
	return requests;
}

export function failedPaymentCodes(rows: ReadonlyArray<EventRow>) {
	return rows.flatMap((row) =>
		isMatching(
			{
				aggregate_type: "payment",
				payload: { type: "payment.failed", retryable: true },
			},
			row,
		)
			? [{ code: row.payload.code, paymentId: row.payload.paymentId }]
			: [],
	);
}

// Order state machine over status tuples.

export type Transition = readonly [from: OrderStatus, to: OrderStatus];

export function checkTransition(transition: Transition) {
	return match(transition)
		.with([P.union("cancelled", "refunded"), P._], () => ({
			ok: false as const,
			reason: "terminal" as const,
		}))
		.with(
			["pending", "pending"],
			["paid", "paid"],
			["packed", "packed"],
			["shipped", "shipped"],
			["delivered", "delivered"],
			() => ({ ok: false as const, reason: "noop" as const }),
		)
		.with(["pending", "paid"], () => ({
			ok: true as const,
			sideEffect: "capture" as const,
		}))
		.with(["paid", "packed"], ["packed", "shipped"], () => ({
			ok: true as const,
			sideEffect: "ship" as const,
		}))
		.with(["shipped", "delivered"], () => ({
			ok: true as const,
			sideEffect: "close" as const,
		}))
		.with(
			[
				P.union("pending", "paid", "packed", "shipped", "delivered"),
				"cancelled",
			],
			[P.union("paid", "packed", "shipped", "delivered"), "refunded"],
			() => ({ ok: true as const, sideEffect: "none" as const }),
		)
		.with(
			["pending", P.union("packed", "shipped", "delivered", "refunded")],
			["paid", P.union("pending", "shipped", "delivered")],
			["packed", P.union("pending", "paid", "delivered")],
			["shipped", P.union("pending", "paid", "packed")],
			["delivered", P.union("pending", "paid", "packed", "shipped")],
			() => ({ ok: false as const, reason: "backwards" as const }),
		)
		.exhaustive();
}

export async function advanceOrder(
	orderId: string,
	version: number,
	from: OrderStatus,
	to: OrderStatus,
) {
	const verdict = checkTransition([from, to]);
	if (!verdict.ok) return verdict;
	const updated = await transitionOrder(orderId, version, to);
	return match(updated)
		.with(P.nullish, () => ({ ok: false as const, reason: "stale" as const }))
		.with({ status: "delivered", fulfilledAt: P.instanceOf(Date) }, (row) => ({
			ok: true as const,
			row,
			closedAt: row.fulfilledAt,
		}))
		.with({ status: P.select() }, (status) => ({
			ok: true as const,
			status,
			sideEffect: verdict.sideEffect,
		}))
		.exhaustive();
}

// Payments.

export function triagePayment(row: AttentionPayment) {
	return match(row)
		.with(
			{
				payload: {
					provider: "stripe",
					outcome: { riskLevel: P.union("elevated", "highest") },
				},
			},
			({ id }) => ({ kind: "manual_review" as const, paymentId: id }),
		)
		.with(
			{
				payload: {
					provider: "adyen",
					resultCode: P.union("Refused", "Error"),
					refusalReason: P.select("reason", P.string.includes("CVC")),
				},
				email: P.select("to"),
			},
			({ to, reason }) => ({
				kind: "email" as const,
				to,
				template: `cvc:${reason}`,
			}),
		)
		.with(
			{
				failureCode: P.string
					.startsWith("card_")
					.or(P.string.startsWith("insufficient")),
			},
			({ email }) => ({
				kind: "email" as const,
				to: email,
				template: "update-card",
			}),
		)
		.with(
			{ status: "authorized", orderStatus: P.union("cancelled", "refunded") },
			({ id }) => ({ kind: "void" as const, paymentId: id }),
		)
		.with(
			{
				provider: P.select("provider"),
				amount: P.number.between(1, 10_000),
				id: P.select("id"),
			},
			({ id, provider }) => ({
				kind: "retry" as const,
				paymentId: id,
				provider,
			}),
		)
		.otherwise(({ id }) => ({ kind: "manual_review" as const, paymentId: id }));
}

export function paymentReference(payload: PaymentPayload) {
	return match(payload)
		.with({ provider: "stripe", chargeId: P.select() }, (id) => ({
			ref: id,
			kind: "charge" as const,
		}))
		.with({ provider: "paypal" }, ({ captureId, payerEmail }) => ({
			ref: captureId,
			kind: "capture" as const,
			contact: payerEmail,
		}))
		.with({ provider: "adyen", refusalReason: P.optional(P.string) }, (p) => ({
			ref: p.pspReference,
			kind: "psp" as const,
			note: p.refusalReason ?? p.resultCode,
		}))
		.exhaustive();
}

export async function buildPaymentQueue() {
	const rows = await paymentsNeedingAttention();
	const actions = rows.map(triagePayment);
	const byKind = {
		review: actions.filter(isMatching({ kind: "manual_review" })),
		retry: actions.filter(
			isMatching({ kind: "retry", provider: P.not("paypal") }),
		),
		emails: actions.filter(
			isMatching({ kind: "email", template: P.string.startsWith("cvc:") }),
		),
	};
	return { rows, actions, byKind } as const;
}

// Order detail classification.

export const isHighValueOrder = isMatching({
	totalCents: P.number.gte(250_000),
	status: P.union("paid", "packed", "shipped"),
	buyer: { country: P.not(P.union("US", "CA")) },
});

export function classifyOrder(order: OrderDetails) {
	return match(order)
		.with({ buyer: P.nullish }, () => ({ segment: "orphaned" as const }))
		.with({ items: [], payments: P.array({ status: "captured" }) }, () => ({
			segment: "empty_but_charged" as const,
		}))
		.with(
			{
				items: P.array({ quantity: P.number.gt(20) }),
				buyer: { lifetimeOrders: P.number.gte(10) },
			},
			(o) => ({ segment: "wholesale" as const, items: o.items.length }),
		)
		.with(
			{
				payments: [
					{ refunds: [{ reason: "fraud" }, ...P.array()] },
					...P.array(),
				],
			},
			(o) => ({ segment: "fraud_refund" as const, orderId: o.id }),
		)
		.with(
			{
				couponKind: P.not(P.nullish).select("coupon"),
				totalCents: P.number.lt(1_000).select("total"),
			},
			({ coupon, total }) => ({
				segment: "coupon_abuse" as const,
				coupon,
				total,
			}),
		)
		.with(
			{
				shipTo: {
					country: P.select("country", P.union("JP", "GB", "DE", "FR", "NL")),
				},
				store: { rating: P.number.between(4.5, 5) },
			},
			({ country }) => ({ segment: "international_premium" as const, country }),
		)
		.with(
			{
				items: P.array({
					attributes: P.record(
						P.string,
						P.union(P.string, P.number, P.boolean),
					),
				}),
			},
			(o) => ({
				segment: "standard" as const,
				lines: o.items.map((i) => i.title),
			}),
		)
		.otherwise(() => ({ segment: "standard" as const, lines: [] as string[] }));
}

export async function orderReport(orderId: string) {
	const order = await orderDetails(orderId);
	const segment = classifyOrder(order);
	const highValue = isHighValueOrder(order);
	const references = order.payments.map((payment) =>
		paymentReference(payment.payload),
	);
	const shipping = match(order.shipTo)
		.with(
			{ country: "US", postalCode: P.string.regex(/^\d{5}$/) },
			(a) => `${a.city}, ${a.postalCode}`,
		)
		.with({ line2: P.string.minLength(1) }, (a) => `${a.line1} / ${a.line2}`)
		.otherwise((a) => `${a.line1}, ${a.city} ${a.country}`);
	return { order, segment, highValue, references, shipping } as const;
}

// Sales dashboards.

export type Tier = "platinum" | "gold" | "silver" | "bronze" | "dormant";

export function storeTier(row: SalesRow): Tier {
	return match(row)
		.returnType<Tier>()
		.with({ orderCount: 0 }, () => "dormant")
		.with(
			{ grossCents: P.number.gte(10_000_000), refundedCount: P.number.lt(5) },
			() => "platinum",
		)
		.with({ grossCents: P.number.between(2_500_000, 10_000_000) }, () => "gold")
		.with(
			{ averageCents: P.number.gt(5_000), currency: P.union("EUR", "GBP") },
			{ refundedCents: P.number.lt(1_000) },
			() => "silver",
		)
		.otherwise(() => "bronze");
}

export async function salesDashboard(range: DateRange, currency?: Currency) {
	const [rows, buckets] = await Promise.all([
		salesByStore({ range, currency, includeRefunds: true, minTotalCents: 100 }),
		revenueByBucket(range),
	]);
	const tiers = rows.map((row) => ({
		store: row.storeName,
		tier: storeTier(row),
	}));
	const summary = buckets.map((bucket: BucketRow) =>
		match(bucket)
			.with({ bucket: "settled" }, (b) => ({
				label: "Settled",
				cents: b.revenueCents,
				positive: true,
			}))
			.with({ bucket: "open" }, (b) => ({
				label: "Open",
				cents: b.revenueCents,
				positive: true,
			}))
			.with({ bucket: P.union("lost", "reversed") }, (b) => ({
				label: b.bucket === "lost" ? "Lost" : "Reversed",
				cents: -b.revenueCents,
				positive: false,
			}))
			.exhaustive(),
	);
	return { tiers, summary } as const;
}

export function pairInsight(row: PairRow) {
	return match(row)
		.with(
			{
				buyerMetadata: {
					acquisition: { channel: "paid", costCents: P.select("cost") },
				},
				orderTotal: P.select("total"),
			},
			({ cost, total }) => ({
				kind: "paid_acquisition" as const,
				roi: total / Math.max(cost, 1),
			}),
		)
		.with(
			{
				buyerMetadata: {
					acquisition: { channel: "referral", code: P.select() },
				},
				referrerName: P.string,
			},
			(code) => ({ kind: "referral" as const, code }),
		)
		.with(
			{ domestic: P.union(false, 0), sellerRole: "seller", completed: true },
			(r) => ({ kind: "cross_border" as const, seller: r.sellerName }),
		)
		.with(
			{
				buyerMetadata: {
					flags: P.when((flags) => flags.includes("fraud_review")),
				},
			},
			(r) => ({
				kind: "review" as const,
				buyer: r.buyerEmail,
			}),
		)
		.otherwise(() => ({ kind: "none" as const }));
}

export function acquisitionLabel(metadata: UserMetadata) {
	return match(metadata.acquisition)
		.with({ channel: "organic" }, () => "Organic")
		.with(
			{ channel: "referral", code: P.string.startsWith("VIP") },
			({ code }) => `VIP referral ${code}`,
		)
		.with({ channel: "referral" }, ({ code }) => `Referral ${code}`)
		.with(
			{ channel: "paid", costCents: P.number.gt(10_000) },
			({ campaign }) => `Expensive: ${campaign}`,
		)
		.with({ channel: "paid" }, ({ campaign }) => `Paid: ${campaign}`)
		.exhaustive();
}

export async function pairReport(range: DateRange) {
	const pairs = await buyerSellerPairs(range);
	return pairs.map((pair) => ({
		pair,
		insight: pairInsight(pair),
		label: acquisitionLabel(pair.buyerMetadata),
	}));
}

// Catalog.

export function stockBadge(product: TopProduct) {
	return match([
		product.stockLevel,
		product.status,
		product.hasOneStar,
	] as const)
		.with(
			["out", "active", P._],
			() => ({ badge: "sold-out", tone: "danger" }) as const,
		)
		.with(
			["low", "active", P.union(true, 1)],
			() => ({ badge: "low-risky", tone: "warning" }) as const,
		)
		.with(
			["low", "active", P._],
			() => ({ badge: "low", tone: "warning" }) as const,
		)
		.with(
			["untracked", P._, P._],
			() => ({ badge: "untracked", tone: "neutral" }) as const,
		)
		.with(
			[P._, "draft", P._],
			() => ({ badge: "draft", tone: "neutral" }) as const,
		)
		.with(
			[
				P.union("ok", "low", "out"),
				P.union("active", "archived", "draft"),
				P._,
			],
			() => ({ badge: "ok", tone: "success" }) as const,
		)
		.exhaustive();
}

export function ratingBand(product: TopProduct) {
	return match(product)
		.with(
			{ avgRating: P.number.between(4.8, 5), reviewCount: P.number.gte(100) },
			() => "top-rated" as const,
		)
		.with(
			{
				avgRating: P.number.gte(4),
				warehouses: P.array(P.string.startsWith("EU-")),
			},
			() => "eu-favourite" as const,
		)
		.with({ avgRating: P.number.gte(4) }, () => "good" as const)
		.with({ available: P.nullish }, () => "unknown" as const)
		.otherwise(() => "average" as const);
}

export interface CategoryBranch {
	readonly node: CategoryNode;
	readonly children: CategoryBranch[];
}

export function nestCategories(
	nodes: ReadonlyArray<CategoryNode>,
): CategoryBranch[] {
	const roots: CategoryBranch[] = [];
	const byId = new Map<number, CategoryBranch>();
	for (const node of nodes) {
		const branch: CategoryBranch = { node, children: [] };
		byId.set(node.id, branch);
		match(node)
			.with({ parentId: P.nullish }, { depth: 0 }, () => roots.push(branch))
			.with({ parentId: P.number }, ({ parentId }) =>
				byId.get(parentId)?.children.push(branch),
			)
			.exhaustive();
	}
	return roots;
}

export async function catalogReport(storeId: string, rootSlug: string) {
	const [products, tree] = await Promise.all([
		topProducts(storeId, 3.5),
		categoryTree(rootSlug),
	]);
	return {
		products: products.map((product) => ({
			id: product.id,
			badge: stockBadge(product),
			band: ratingBand(product),
		})),
		tree: nestCategories(tree),
	} as const;
}

// Event loop tying it together.

export type ProcessedEvent = {
	readonly id: string;
	readonly action: EventAction;
	readonly text: string;
};

export async function drainEvents(pageSize: number) {
	const processed: ProcessedEvent[] = [];
	let cursor: string | null = null;
	for (;;) {
		const page = await eventFeed(cursor, pageSize);
		const last = match(page)
			.with([], () => null)
			.with([...P.array(), P.select()], (tail) => tail)
			.otherwise(() => null);
		for (const row of page) {
			processed.push({
				id: row.id,
				action: routeEvent(row.payload),
				text: describeEvent(row),
			});
		}
		if (last === null) break;
		cursor = last.id;
	}
	return processed;
}

export async function safePlaceOrder(input: PlaceOrderInput) {
	try {
		const placed = await placeOrder(input);
		return match(placed.reservations)
			.with(P.array(P.nonNullable), () => ({ ok: true as const, placed }))
			.otherwise((reservations) => ({
				ok: false as const,
				missing: reservations.flatMap((r, i) =>
					r ? [] : [placed.items[i]?.product_id],
				),
			}));
	} catch (error) {
		if (error instanceof NonExhaustiveError) throw error;
		return { ok: false as const, recovery: classifyError(error) };
	}
}

export function regionFor(country: CountryCode) {
	return match(country)
		.with("US", "CA", () => "NA" as const)
		.with("GB", "DE", "FR", "NL", () => "EU" as const)
		.with("JP", () => "APAC" as const)
		.exhaustive();
}

function formatMoney(cents: number, currency: Currency) {
	return match(currency)
		.with("JPY", () => `¥${cents}`)
		.with(
			P.union("USD", "CAD"),
			(c) => `${c === "USD" ? "$" : "C$"}${(cents / 100).toFixed(2)}`,
		)
		.with("EUR", () => `€${(cents / 100).toFixed(2)}`)
		.with("GBP", () => `£${(cents / 100).toFixed(2)}`)
		.exhaustive();
}
