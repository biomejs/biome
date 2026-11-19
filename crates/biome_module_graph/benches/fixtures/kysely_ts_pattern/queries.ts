import {
	sql,
	jsonArrayFrom,
	jsonBuildObject,
	jsonObjectFrom,
	type Expression,
	type ExpressionBuilder,
	type NotNull,
	type SqlBool,
	type Transaction,
} from "./vendor/kysely";
import {
	db,
	type Currency,
	type Database,
	type NewDailySales,
	type NewOrder,
	type NewOrderItem,
	type OrderStatus,
	type ProductUpdate,
	type UserRole,
} from "./schema";

export interface DateRange {
	readonly from: Date;
	readonly to: Date;
}

export interface SalesFilter {
	readonly range: DateRange;
	readonly storeIds?: ReadonlyArray<string>;
	readonly currency?: Currency;
	readonly statuses?: ReadonlyArray<OrderStatus>;
	readonly includeRefunds?: boolean;
	readonly minTotalCents?: number;
}

const settledStatuses = ["paid", "packed", "shipped", "delivered"] as const;

// Reusable expression fragments.

export function isSettled(eb: ExpressionBuilder<Database, "orders">) {
	return eb("orders.status", "in", settledStatuses);
}

export function orderRevenue(eb: ExpressionBuilder<Database, "orders">) {
	return eb.fn
		.coalesce(eb.fn.sum<number>("orders.total_cents"), eb.lit(0))
		.as("revenueCents");
}

export function netItemTotal(eb: ExpressionBuilder<Database, "order_items">) {
	return eb(
		eb("order_items.unit_price_cents", "*", eb.ref("order_items.quantity")),
		"-",
		eb.ref("order_items.discount_cents"),
	);
}

export function statusBucket(eb: ExpressionBuilder<Database, "orders">) {
	return eb
		.case()
		.when("orders.status", "in", ["pending"])
		.then("open" as const)
		.when("orders.status", "in", settledStatuses)
		.then("settled" as const)
		.when("orders.status", "=", "cancelled")
		.then("lost" as const)
		.else("reversed" as const)
		.end()
		.as("bucket");
}

export function placedWithin(
	eb: ExpressionBuilder<Database, "orders">,
	range: DateRange,
): Expression<SqlBool> {
	return eb.and([
		eb("orders.placed_at", ">=", range.from),
		eb("orders.placed_at", "<", range.to),
	]);
}

export function buyerProfile(
	eb: ExpressionBuilder<Database & { o: Database["orders"] }, "o">,
) {
	return jsonObjectFrom(
		eb
			.selectFrom("users as u")
			.select([
				"u.id",
				"u.display_name as name",
				"u.country_code as country",
				(inner) =>
					inner
						.selectFrom("orders as prior")
						.select((p) => p.fn.countAll<number>().as("n"))
						.whereRef("prior.buyer_id", "=", "u.id")
						.as("lifetimeOrders"),
			])
			.whereRef("u.id", "=", "o.buyer_id"),
	).as("buyer");
}

// Read models.

export async function salesByStore(filter: SalesFilter) {
	return await db
		.selectFrom("orders as o")
		.innerJoin("stores as s", "s.id", "o.store_id")
		.innerJoin("users as seller", "seller.id", "s.owner_id")
		.select((eb) => [
			"s.id as storeId",
			"s.name as storeName",
			"seller.display_name as sellerName",
			"o.currency",
			eb.fn.count<number>("o.id").as("orderCount"),
			eb.fn.sum<number>("o.total_cents").as("grossCents"),
			eb.fn.avg<number>("o.total_cents").as("averageCents"),
			eb.fn.max("o.placed_at").as("lastOrderAt"),
			eb.fn
				.count<number>("o.id")
				.filterWhere("o.status", "=", "refunded")
				.as("refundedCount"),
		])
		.where((eb) =>
			eb.and([
				eb("o.placed_at", ">=", filter.range.from),
				eb("o.placed_at", "<", filter.range.to),
			]),
		)
		.$if(filter.storeIds !== undefined, (qb) =>
			qb.where("o.store_id", "in", filter.storeIds ?? []),
		)
		.$if(filter.currency !== undefined, (qb) =>
			qb.where("o.currency", "=", filter.currency ?? "USD"),
		)
		.$if(filter.statuses !== undefined, (qb) =>
			qb.where("o.status", "in", filter.statuses ?? settledStatuses),
		)
		.$if(filter.includeRefunds === true, (qb) =>
			qb
				.leftJoin("payments as p", "p.order_id", "o.id")
				.leftJoin("refunds as r", "r.payment_id", "p.id")
				.select((eb) =>
					eb.fn
						.coalesce(eb.fn.sum<number>("r.amount_cents"), sql.lit(0))
						.as("refundedCents"),
				),
		)
		.groupBy(["s.id", "s.name", "seller.display_name", "o.currency"])
		.having((eb) => eb.fn.count("o.id"), ">", 0)
		.$if(filter.minTotalCents !== undefined, (qb) =>
			qb.having(
				(eb) => eb.fn.sum("o.total_cents"),
				">=",
				filter.minTotalCents ?? 0,
			),
		)
		.orderBy("grossCents", "desc")
		.execute();
}

export async function revenueByBucket(range: DateRange) {
	return await db
		.selectFrom("orders")
		.select((eb) => [
			statusBucket(eb),
			orderRevenue(eb),
			eb.fn.countAll<number>().as("orders"),
			eb.fn.count<number>("orders.buyer_id").distinct().as("buyers"),
		])
		.where((eb) => placedWithin(eb, range))
		.groupBy("bucket")
		.execute();
}

export async function settledLineItems(storeId: string) {
	return await db
		.selectFrom("order_items")
		.innerJoin("orders", "orders.id", "order_items.order_id")
		.select((eb) => [
			"order_items.product_id as productId",
			eb.fn.sum<number>("order_items.quantity").as("units"),
			eb.fn.sum<number>(netItemTotal(eb)).as("netCents"),
		])
		.where("orders.store_id", "=", storeId)
		.where(isSettled)
		.groupBy("order_items.product_id")
		.execute();
}

export async function buyerSellerPairs(range: DateRange, limit = 50) {
	return await db
		.selectFrom("orders as o")
		.innerJoin("users as buyer", "buyer.id", "o.buyer_id")
		.innerJoin("users as seller", "seller.id", "o.seller_id")
		.leftJoin("users as referrer", "referrer.id", "buyer.referred_by")
		.select([
			"buyer.id as buyerId",
			"buyer.email as buyerEmail",
			"buyer.metadata as buyerMetadata",
			"seller.id as sellerId",
			"seller.display_name as sellerName",
			"seller.role as sellerRole",
			"referrer.display_name as referrerName",
			"o.total_cents as orderTotal",
			"o.status",
		])
		.select((eb) => [
			eb("buyer.country_code", "=", eb.ref("seller.country_code")).as(
				"domestic",
			),
			eb
				.case("o.status")
				.when("delivered")
				.then(true)
				.else(false)
				.end()
				.as("completed"),
		])
		.where((eb) => eb.between("o.placed_at", range.from, range.to))
		.where("buyer.id", "!=", (eb) => eb.ref("seller.id"))
		.orderBy(["o.total_cents desc", "buyer.id"])
		.limit(limit)
		.execute();
}

export async function orderDetails(orderId: string) {
	return await db
		.selectFrom("orders as o")
		.innerJoin("stores as s", "s.id", "o.store_id")
		.leftJoin("coupons as c", "c.id", "o.coupon_id")
		.select((eb) => [
			"o.id",
			"o.status",
			"o.currency",
			"o.total_cents as totalCents",
			"o.shipping_address as shipTo",
			"o.placed_at as placedAt",
			"o.fulfilled_at as fulfilledAt",
			"s.name as storeName",
			"c.code as couponCode",
			"c.kind as couponKind",
			buyerProfile(eb),
			jsonArrayFrom(
				eb
					.selectFrom("order_items as i")
					.innerJoin("products as p", "p.id", "i.product_id")
					.leftJoin("categories as cat", "cat.id", "p.category_id")
					.select((ib) => [
						"i.id as itemId",
						"i.quantity",
						"i.unit_price_cents as unitPrice",
						"p.title",
						"p.sku",
						"p.attributes",
						"cat.name as categoryName",
						ib(
							ib("i.unit_price_cents", "*", ib.ref("i.quantity")),
							"-",
							ib.ref("i.discount_cents"),
						).as("lineTotal"),
					])
					.whereRef("i.order_id", "=", "o.id")
					.orderBy("i.id"),
			).as("items"),
			jsonArrayFrom(
				eb
					.selectFrom("payments as pay")
					.select([
						"pay.id",
						"pay.provider",
						"pay.status",
						"pay.amount_cents as amount",
						"pay.failure_code as failureCode",
						"pay.payload",
					])
					.select((pb) =>
						jsonArrayFrom(
							pb
								.selectFrom("refunds as rf")
								.select(["rf.id", "rf.amount_cents as amount", "rf.reason"])
								.whereRef("rf.payment_id", "=", "pay.id"),
						).as("refunds"),
					)
					.whereRef("pay.order_id", "=", "o.id"),
			).as("payments"),
			jsonBuildObject({
				rating: eb.ref("s.rating"),
				slug: eb.ref("s.slug"),
			}).as("store"),
		])
		.where("o.id", "=", orderId)
		.executeTakeFirstOrThrow();
}

export async function categoryTree(rootSlug: string) {
	return await db
		.withRecursive("tree(id, parent_id, name, slug, depth, path)", (qc) =>
			qc
				.selectFrom("categories")
				.select((eb) => [
					"id",
					"parent_id",
					"name",
					"slug",
					eb.val(0).as("depth"),
					eb.cast<string>("slug", "text").as("path"),
				])
				.where("slug", "=", rootSlug)
				.unionAll((ub) =>
					ub
						.selectFrom("categories as child")
						.innerJoin("tree", "tree.id", "child.parent_id")
						.select((eb) => [
							"child.id",
							"child.parent_id",
							"child.name",
							"child.slug",
							eb("tree.depth", "+", 1).as("depth"),
							sql<string>`${eb.ref("tree.path")} || '/' || ${eb.ref("child.slug")}`.as(
								"path",
							),
						]),
				),
		)
		.with("category_sales", (qc) =>
			qc
				.selectFrom("order_items as i")
				.innerJoin("products as p", "p.id", "i.product_id")
				.innerJoin("orders as o", "o.id", "i.order_id")
				.select((eb) => [
					"p.category_id",
					eb.fn.sum<number>("i.quantity").as("units"),
					eb.fn
						.sum<number>(
							eb(
								eb("i.unit_price_cents", "*", eb.ref("i.quantity")),
								"-",
								eb.ref("i.discount_cents"),
							),
						)
						.as("netCents"),
				])
				.where("o.status", "in", settledStatuses)
				.groupBy("p.category_id"),
		)
		.selectFrom("tree")
		.leftJoin("category_sales as cs", "cs.category_id", "tree.id")
		.select((eb) => [
			"tree.id",
			"tree.parent_id as parentId",
			"tree.name",
			"tree.depth",
			"tree.path",
			eb.fn.coalesce("cs.units", eb.lit(0)).as("units"),
			eb.fn.coalesce("cs.netCents", eb.lit(0)).as("netCents"),
		])
		.orderBy("tree.path")
		.execute();
}

export async function topProducts(storeId: string, minRating: number) {
	return await db
		.with("rated", (qc) =>
			qc
				.selectFrom("reviews")
				.select((eb) => [
					"product_id",
					eb.fn.avg<number>("rating").as("avgRating"),
					eb.fn.countAll<number>().as("reviewCount"),
				])
				.groupBy("product_id")
				.having((eb) => eb.fn.avg("rating"), ">=", minRating),
		)
		.with("stock", (qc) =>
			qc
				.selectFrom("inventory")
				.innerJoin("warehouses as w", "w.id", "inventory.warehouse_id")
				.select((eb) => [
					"inventory.product_id",
					eb.fn
						.sum<number>(
							eb("inventory.quantity", "-", eb.ref("inventory.reserved")),
						)
						.as("available"),
					eb.fn.agg<string[]>("array_agg", ["w.code"]).as("warehouses"),
				])
				.groupBy("inventory.product_id"),
		)
		.selectFrom("products as p")
		.innerJoin("rated", "rated.product_id", "p.id")
		.leftJoin("stock", "stock.product_id", "p.id")
		.select((eb) => [
			"p.id",
			"p.title",
			"p.price_cents as priceCents",
			"p.status",
			"rated.avgRating",
			"rated.reviewCount",
			"stock.available",
			"stock.warehouses",
			eb
				.case()
				.when("stock.available", "is", null)
				.then("untracked" as const)
				.when("stock.available", "<=", 0)
				.then("out" as const)
				.when("stock.available", "<", 10)
				.then("low" as const)
				.else("ok" as const)
				.end()
				.as("stockLevel"),
			eb
				.exists(
					eb
						.selectFrom("reviews as bad")
						.select("bad.id")
						.whereRef("bad.product_id", "=", "p.id")
						.where("bad.rating", "=", 1),
				)
				.as("hasOneStar"),
		])
		.where("p.store_id", "=", storeId)
		.where("p.status", "!=", "archived")
		.orderBy("rated.avgRating", "desc")
		.limit(25)
		.execute();
}

export async function paymentsNeedingAttention() {
	const rows = await db
		.selectFrom("payments as p")
		.innerJoin("orders as o", "o.id", "p.order_id")
		.innerJoin("users as buyer", "buyer.id", "o.buyer_id")
		.select([
			"p.id",
			"p.provider",
			"p.status",
			"p.failure_code as failureCode",
			"p.amount_cents as amount",
			"p.payload",
			"o.id as orderId",
			"o.status as orderStatus",
			"buyer.email",
		])
		.where((eb) =>
			eb.or([
				eb("p.status", "=", "failed"),
				eb.and([
					eb("p.status", "=", "authorized"),
					eb("p.created_at", "<", sql<Date>`now() - interval '3 days'`),
				]),
			]),
		)
		.where("p.failure_code", "is not", null)
		.$narrowType<{ failureCode: NotNull }>()
		.execute();
	return rows;
}

export async function cohortRetention(role: UserRole, months: number) {
	return await db
		.with("cohorts", (qc) =>
			qc
				.selectFrom("users")
				.select([
					"id as userId",
					sql<string>`date_trunc('month', created_at)`.as("cohort"),
				])
				.where("role", "=", role),
		)
		.with("activity", (qc) =>
			qc
				.selectFrom("orders")
				.innerJoin("cohorts", "cohorts.userId", "orders.buyer_id")
				.select([
					"cohorts.cohort",
					"orders.buyer_id",
					sql<number>`extract(month from age(orders.placed_at, cohorts.cohort::date))`.as(
						"monthOffset",
					),
				])
				.distinct(),
		)
		.selectFrom("activity")
		.select((eb) => [
			"activity.cohort",
			"activity.monthOffset",
			eb.fn.count<number>("activity.buyer_id").distinct().as("activeBuyers"),
		])
		.where("activity.monthOffset", "<=", months)
		.groupBy(["activity.cohort", "activity.monthOffset"])
		.orderBy(["activity.cohort", "activity.monthOffset"])
		.execute();
}

export async function eventFeed(after: string | null, pageSize: number) {
	return await db
		.selectFrom("events")
		.selectAll()
		.$if(after !== null, (qb) => qb.where("id", ">", after ?? ""))
		.where("processed_at", "is", null)
		.orderBy("id")
		.limit(pageSize)
		.execute();
}

// Writes.

export async function upsertDailySales(rows: ReadonlyArray<NewDailySales>) {
	return await db
		.insertInto("daily_sales")
		.values([...rows])
		.onConflict((oc) =>
			oc.columns(["store_id", "day"]).doUpdateSet((eb) => ({
				gross_cents: eb(
					"daily_sales.gross_cents",
					"+",
					eb.ref("excluded.gross_cents"),
				),
				refunded_cents: eb(
					"daily_sales.refunded_cents",
					"+",
					eb.ref("excluded.refunded_cents"),
				),
				order_count: eb(
					"daily_sales.order_count",
					"+",
					eb.ref("excluded.order_count"),
				),
				updated_at: new Date(),
			})),
		)
		.returning(["store_id as storeId", "day", "gross_cents as grossCents"])
		.execute();
}

export async function rollupDay(day: string) {
	return await db
		.insertInto("daily_sales")
		.columns([
			"store_id",
			"day",
			"gross_cents",
			"refunded_cents",
			"order_count",
			"updated_at",
		])
		.expression((eb) =>
			eb
				.selectFrom("orders as o")
				.leftJoin("payments as p", "p.order_id", "o.id")
				.leftJoin("refunds as r", "r.payment_id", "p.id")
				.select((sb) => [
					"o.store_id",
					sb.val(day).as("day"),
					sb.fn
						.coalesce(sb.fn.sum<number>("o.total_cents"), sb.lit(0))
						.as("gross_cents"),
					sb.fn
						.coalesce(sb.fn.sum<number>("r.amount_cents"), sb.lit(0))
						.as("refunded_cents"),
					sb.fn.count<number>("o.id").distinct().as("order_count"),
					sql<Date>`now()`.as("updated_at"),
				])
				.where(sql<string>`o.placed_at::date`, "=", day)
				.groupBy("o.store_id"),
		)
		.onConflict((oc) => oc.columns(["store_id", "day"]).doNothing())
		.executeTakeFirst();
}

export async function transitionOrder(
	orderId: string,
	expectedVersion: number,
	next: OrderStatus,
) {
	return await db
		.updateTable("orders")
		.set((eb) => ({
			status: next,
			version: eb("version", "+", 1),
			fulfilled_at: next === "delivered" ? new Date() : null,
		}))
		.where("id", "=", orderId)
		.where("version", "=", expectedVersion)
		.returning(["id", "status", "version", "fulfilled_at as fulfilledAt"])
		.executeTakeFirst();
}

export async function repriceStore(storeId: string, percent: number) {
	const patch: ProductUpdate = { updated_at: new Date() };
	return await db
		.updateTable("products as p")
		.set(patch)
		.set((eb) => ({
			price_cents: eb.cast<number>(
				eb("p.price_cents", "*", 1 + percent / 100),
				"integer",
			),
		}))
		.where("p.store_id", "=", storeId)
		.where("p.status", "=", "active")
		.returning(["p.id", "p.sku", "p.price_cents as newPrice"])
		.execute();
}

export interface PlaceOrderInput {
	readonly order: NewOrder;
	readonly items: ReadonlyArray<Omit<NewOrderItem, "order_id">>;
	readonly warehouseId: number;
}

async function reserveStock(
	trx: Transaction<Database>,
	warehouseId: number,
	productId: string,
	quantity: number,
) {
	return await trx
		.updateTable("inventory")
		.set((eb) => ({ reserved: eb("reserved", "+", quantity) }))
		.where("product_id", "=", productId)
		.where("warehouse_id", "=", warehouseId)
		.where((eb) => eb(eb("quantity", "-", eb.ref("reserved")), ">=", quantity))
		.returning(["product_id as productId", "quantity", "reserved"])
		.executeTakeFirst();
}

export async function placeOrder(input: PlaceOrderInput) {
	return await db.transaction().execute(async (trx) => {
		const order = await trx
			.insertInto("orders")
			.values(input.order)
			.returning(["id", "status", "total_cents", "currency", "placed_at"])
			.executeTakeFirstOrThrow();

		const items = await trx
			.insertInto("order_items")
			.values(input.items.map((item) => ({ ...item, order_id: order.id })))
			.returningAll()
			.execute();

		const reservations = await Promise.all(
			items.map((item) =>
				reserveStock(trx, input.warehouseId, item.product_id, item.quantity),
			),
		);

		await trx
			.insertInto("events")
			.values({
				aggregate_type: "order",
				aggregate_id: order.id,
				payload: JSON.stringify({
					type: "order.placed",
					orderId: order.id,
					totalCents: order.total_cents,
					currency: order.currency,
				}),
			})
			.execute();

		return { order, items, reservations } as const;
	});
}

export async function markEventsProcessed(ids: ReadonlyArray<string>) {
	const result = await db
		.updateTable("events")
		.set({ processed_at: new Date() })
		.where("id", "in", ids)
		.executeTakeFirst();
	return Number(result.numUpdatedRows);
}

export async function purgeAbandoned(before: Date) {
	return await db
		.deleteFrom("orders")
		.where("status", "=", "pending")
		.where("placed_at", "<", before)
		.where((eb) =>
			eb.not(
				eb.exists(
					eb
						.selectFrom("payments")
						.select("payments.id")
						.whereRef("payments.order_id", "=", "orders.id"),
				),
			),
		)
		.returning(["id", "buyer_id as buyerId"])
		.execute();
}

export type BucketRow = Awaited<ReturnType<typeof revenueByBucket>>[number];
export type LineItemRow = Awaited<ReturnType<typeof settledLineItems>>[number];
export type SalesRow = Awaited<ReturnType<typeof salesByStore>>[number];
export type PairRow = Awaited<ReturnType<typeof buyerSellerPairs>>[number];
export type OrderDetails = Awaited<ReturnType<typeof orderDetails>>;
export type CategoryNode = Awaited<ReturnType<typeof categoryTree>>[number];
export type TopProduct = Awaited<ReturnType<typeof topProducts>>[number];
export type AttentionPayment = Awaited<
	ReturnType<typeof paymentsNeedingAttention>
>[number];
export type RetentionRow = Awaited<ReturnType<typeof cohortRetention>>[number];
export type EventRow = Awaited<ReturnType<typeof eventFeed>>[number];
export type PlacedOrder = Awaited<ReturnType<typeof placeOrder>>;
