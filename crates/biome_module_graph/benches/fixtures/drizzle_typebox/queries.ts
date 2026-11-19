import {
	eq,
	and,
	or,
	desc,
	asc,
	count,
	sum,
	sql,
	inArray,
	gte,
	alias,
	getTableColumns,
	Type,
	type Static,
	type PgDatabase,
	type PgQueryResultHKT,
} from "./vendor/drizzle-typebox";
import * as schema from "./schema";
import {
	organizations,
	users,
	memberships,
	products,
	orders,
	orderItems,
	shipments,
	productSchema,
	type Search,
	type Checkout,
	type OrderPatch,
} from "./schema";

export declare const db: PgDatabase<PgQueryResultHKT, typeof schema>;
const approvers = alias(users, "approvers");
export function searchOrders(input: Search) {
	const totals = db.$with("item_totals").as(
		db
			.select({
				orderId: orderItems.orderId,
				quantity: sum(orderItems.quantity).as("quantity"),
				value:
					sql<number>`sum(${orderItems.quantity} * ${orderItems.unitPrice})`
						.mapWith(Number)
						.as("value"),
			})
			.from(orderItems)
			.groupBy(orderItems.orderId),
	);
	return db
		.with(totals)
		.select({
			order: {
				...getTableColumns(orders),
				itemQuantity: totals.quantity,
				computedTotal: totals.value,
			},
			organization: {
				id: organizations.id,
				name: organizations.name,
				settings: organizations.settings,
			},
			buyer: { id: users.id, name: users.name, profile: users.profile },
			approver: { id: approvers.id, email: approvers.email },
		})
		.from(orders)
		.innerJoin(organizations, eq(orders.organizationId, organizations.id))
		.innerJoin(users, eq(orders.buyerId, users.id))
		.leftJoin(approvers, eq(orders.approverId, approvers.id))
		.leftJoin(totals, eq(totals.orderId, orders.id))
		.where(
			and(
				eq(orders.organizationId, input.organizationId),
				inArray(orders.status, input.statuses),
				input.minimum === undefined
					? undefined
					: gte(totals.value, input.minimum),
			),
		)
		.orderBy(desc(orders.placedAt), asc(orders.id))
		.limit(input.pagination.limit)
		.offset(input.pagination.offset);
}
export function orderDetails(organizationId: string) {
	return db.query.orders.findMany({
		where: (order, { eq }) => eq(order.organizationId, organizationId),
		columns: {
			id: true,
			status: true,
			total: true,
			metadata: true,
			placedAt: true,
		},
		with: {
			buyer: {
				columns: { id: true, email: true, profile: true },
				with: {
					memberships: {
						with: { organization: { columns: { name: true, settings: true } } },
					},
				},
			},
			approver: { columns: { id: true, name: true } },
			items: {
				with: {
					product: {
						with: { organization: { columns: { id: true, name: true } } },
					},
				},
			},
			shipments: {
				orderBy: (shipment, { desc }) => [desc(shipment.deliveredAt)],
				limit: 10,
			},
		},
		extras: (order, { sql }) => ({
			display: sql<string>`${order.id} || ':' || ${order.status}`.as("display"),
		}),
		orderBy: (order, { desc }) => [desc(order.placedAt)],
		limit: 50,
	});
}
export function inventoryReport(organizationId: string) {
	return db
		.select({
			product: {
				id: products.id,
				sku: products.sku,
				specifications: products.specifications,
			},
			orders: count(orders.id),
			quantity: sum(orderItems.quantity),
		})
		.from(products)
		.leftJoin(orderItems, eq(products.id, orderItems.productId))
		.leftJoin(orders, eq(orderItems.orderId, orders.id))
		.where(
			and(
				eq(products.organizationId, organizationId),
				or(eq(orders.status, "paid"), eq(orders.status, "shipped")),
			),
		)
		.groupBy(products.id)
		.having(({ orders }) => gte(orders, 5))
		.orderBy(desc(count(orders.id)));
}
export async function checkout(input: Checkout) {
	return db.transaction(async (transaction) => {
		const { items, approval, ...order } = input;
		const [created] = await transaction
			.insert(orders)
			.values({
				...order,
				total: items
					.reduce(
						(sum, item) => sum + Number(item.unitPrice) * item.quantity,
						0,
					)
					.toFixed(2),
			})
			.returning({
				id: orders.id,
				status: orders.status,
				metadata: orders.metadata,
			});
		const inserted = await transaction
			.insert(orderItems)
			.values(items.map((item) => ({ ...item, orderId: created.id })))
			.returning();
		const membership = await transaction
			.select({ role: memberships.role, permissions: memberships.permissions })
			.from(memberships)
			.where(
				and(
					eq(memberships.organizationId, input.organizationId),
					eq(memberships.userId, input.buyerId),
				),
			);
		return { order: created, items: inserted, approval, membership };
	});
}
export function patchOrder(patch: OrderPatch) {
	const { id, ...values } = patch;
	return db
		.update(orders)
		.set(values)
		.where(eq(orders.id, id))
		.returning({ id: orders.id, status: orders.status, total: orders.total });
}
export const shipmentReport = db
	.select({
		orderId: orders.id,
		tracking: shipments.tracking,
		events: shipments.events,
		buyerEmail: users.email,
	})
	.from(shipments)
	.innerJoin(orders, eq(shipments.orderId, orders.id))
	.innerJoin(users, eq(orders.buyerId, users.id));
export type SearchRows = Awaited<ReturnType<typeof searchOrders>>;
export type DetailRows = Awaited<ReturnType<typeof orderDetails>>;
export type InventoryRows = Awaited<ReturnType<typeof inventoryReport>>;
export type CheckoutResult = Awaited<ReturnType<typeof checkout>>;
export type ProductDetails = DetailRows[number]["items"][number]["product"];
export type BuyerOrganizations =
	DetailRows[number]["buyer"]["memberships"][number]["organization"];
export type Reports = {
	search: SearchRows;
	details: DetailRows;
	inventory: InventoryRows;
	checkout: CheckoutResult;
};
export type ReportRow<Key extends keyof Reports> =
	Reports[Key] extends readonly (infer Row)[] ? Row : Reports[Key];
export const inventoryResponseSchema = Type.Object({
	data: Type.Array(
		Type.Intersect([
			Type.Pick(productSchema, ["id", "sku", "specifications"]),
			Type.Object({
				quantity: Type.Union([Type.String(), Type.Null()]),
				orders: Type.Integer(),
			}),
		]),
	),
	pagination: Type.Object({ next: Type.Union([Type.String(), Type.Null()]) }),
});
export type InventoryResponse = Static<typeof inventoryResponseSchema>;
export declare const search: Search;
export const orderQuery = searchOrders(search);
export const detailQuery = orderDetails(search.organizationId);
export const stockQuery = inventoryReport(search.organizationId);
export async function dashboard() {
	const [orders, details, inventory] = await Promise.all([
		orderQuery,
		detailQuery,
		stockQuery,
	]);
	return {
		orders,
		buyers: details.map((order) => order.buyer.email),
		products: details.flatMap((order) =>
			order.items.map((item) => item.product),
		),
		inventory,
	};
}
