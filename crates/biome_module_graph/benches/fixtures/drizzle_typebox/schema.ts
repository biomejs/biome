import {
	pgTable,
	pgEnum,
	uuid,
	text,
	integer,
	numeric,
	boolean,
	timestamp,
	jsonb,
	primaryKey,
	index,
	relations,
	Type,
	createSelectSchema,
	createInsertSchema,
	createUpdateSchema,
	type Static,
} from "./vendor/drizzle-typebox";

export const orderStatus = pgEnum("order_status", [
	"draft",
	"submitted",
	"paid",
	"shipped",
	"cancelled",
]);
export const organizations = pgTable("organizations", {
	id: uuid().defaultRandom().primaryKey(),
	name: text().notNull(),
	settings: jsonb()
		.$type<{
			currency: "USD" | "EUR" | "GBP";
			billing: {
				taxId?: string;
				address: { city: string; country: string; lines: string[] };
			};
			features: Record<string, boolean>;
		}>()
		.notNull(),
	createdAt: timestamp({ withTimezone: true }).defaultNow().notNull(),
});
export const users = pgTable("users", {
	id: uuid().defaultRandom().primaryKey(),
	email: text().notNull().unique(),
	name: text().notNull(),
	profile: jsonb().$type<{
		locale: string;
		notifications: { email: boolean; channels: string[] };
		contacts: { kind: "phone" | "email"; value: string }[];
	}>(),
	active: boolean().default(true).notNull(),
});
export const memberships = pgTable(
	"memberships",
	{
		organizationId: uuid()
			.notNull()
			.references(() => organizations.id),
		userId: uuid()
			.notNull()
			.references(() => users.id),
		role: text({ enum: ["owner", "admin", "buyer", "viewer"] }).notNull(),
		permissions: jsonb()
			.$type<{
				resources: string[];
				limits: { daily: number; monthly: number };
			}>()
			.notNull(),
	},
	(table) => [primaryKey({ columns: [table.organizationId, table.userId] })],
);
export const products = pgTable("products", {
	id: uuid().defaultRandom().primaryKey(),
	organizationId: uuid()
		.notNull()
		.references(() => organizations.id),
	sku: text().notNull().unique(),
	name: text().notNull(),
	price: numeric({ precision: 12, scale: 2 }).notNull(),
	tags: text().array().notNull(),
	specifications: jsonb()
		.$type<{
			variants: {
				code: string;
				dimensions: [number, number, number];
				options: Record<string, string>;
			}[];
			warehouse: { region: string; bins: string[] };
		}>()
		.notNull(),
});
export const orders = pgTable(
	"orders",
	{
		id: uuid().defaultRandom().primaryKey(),
		organizationId: uuid()
			.notNull()
			.references(() => organizations.id),
		buyerId: uuid()
			.notNull()
			.references(() => users.id),
		approverId: uuid().references(() => users.id),
		status: orderStatus().default("draft").notNull(),
		total: numeric({ precision: 14, scale: 2 }).notNull(),
		metadata: jsonb()
			.$type<{
				source: "web" | "api";
				labels: string[];
				shipping: { address: string[]; instructions?: string };
				discounts: { code: string; amount: number }[];
			}>()
			.notNull(),
		placedAt: timestamp({ withTimezone: true }).defaultNow().notNull(),
	},
	(table) => [
		index("orders_organization_status").on(table.organizationId, table.status),
	],
);
export const orderItems = pgTable(
	"order_items",
	{
		orderId: uuid()
			.notNull()
			.references(() => orders.id),
		productId: uuid()
			.notNull()
			.references(() => products.id),
		quantity: integer().notNull(),
		unitPrice: numeric({ precision: 12, scale: 2 }).notNull(),
		configuration: jsonb().$type<{
			variant: string;
			options: Record<string, string>;
			engraving?: { text: string; font: string };
		}>(),
	},
	(table) => [primaryKey({ columns: [table.orderId, table.productId] })],
);
export const shipments = pgTable("shipments", {
	id: uuid().defaultRandom().primaryKey(),
	orderId: uuid()
		.notNull()
		.references(() => orders.id),
	tracking: text(),
	carrier: text({ enum: ["postal", "courier", "freight"] }).notNull(),
	events: jsonb()
		.$type<
			{
				timestamp: string;
				location: { city: string; country: string };
				status: string;
			}[]
		>()
		.notNull(),
	deliveredAt: timestamp({ withTimezone: true }),
});

export const organizationRelations = relations(organizations, ({ many }) => ({
	memberships: many(memberships),
	products: many(products),
	orders: many(orders),
}));
export const userRelations = relations(users, ({ many }) => ({
	memberships: many(memberships),
	purchases: many(orders, { relationName: "buyer" }),
	approvals: many(orders, { relationName: "approver" }),
}));
export const membershipRelations = relations(memberships, ({ one }) => ({
	organization: one(organizations, {
		fields: [memberships.organizationId],
		references: [organizations.id],
	}),
	user: one(users, { fields: [memberships.userId], references: [users.id] }),
}));
export const productRelations = relations(products, ({ one, many }) => ({
	organization: one(organizations, {
		fields: [products.organizationId],
		references: [organizations.id],
	}),
	items: many(orderItems),
}));
export const orderRelations = relations(orders, ({ one, many }) => ({
	organization: one(organizations, {
		fields: [orders.organizationId],
		references: [organizations.id],
	}),
	buyer: one(users, {
		fields: [orders.buyerId],
		references: [users.id],
		relationName: "buyer",
	}),
	approver: one(users, {
		fields: [orders.approverId],
		references: [users.id],
		relationName: "approver",
	}),
	items: many(orderItems),
	shipments: many(shipments),
}));
export const itemRelations = relations(orderItems, ({ one }) => ({
	order: one(orders, { fields: [orderItems.orderId], references: [orders.id] }),
	product: one(products, {
		fields: [orderItems.productId],
		references: [products.id],
	}),
}));
export const shipmentRelations = relations(shipments, ({ one }) => ({
	order: one(orders, { fields: [shipments.orderId], references: [orders.id] }),
}));

export const productSchema = createSelectSchema(products, {
	specifications: Type.Object({
		variants: Type.Array(
			Type.Object({
				code: Type.String(),
				dimensions: Type.Tuple([Type.Number(), Type.Number(), Type.Number()]),
				options: Type.Record(Type.String(), Type.String()),
			}),
		),
		warehouse: Type.Object({
			region: Type.String(),
			bins: Type.Array(Type.String()),
		}),
	}),
});
export const insertOrderSchema = createInsertSchema(orders, {
	metadata: Type.Object({
		source: Type.Union([Type.Literal("web"), Type.Literal("api")]),
		labels: Type.Array(Type.String()),
		shipping: Type.Object({
			address: Type.Array(Type.String()),
			instructions: Type.Optional(Type.String()),
		}),
		discounts: Type.Array(
			Type.Object({ code: Type.String(), amount: Type.Number({ minimum: 0 }) }),
		),
	}),
});
export const updateOrderSchema = createUpdateSchema(orders);
export const insertItemSchema = createInsertSchema(orderItems, {
	quantity: Type.Integer({ minimum: 1, maximum: 10000 }),
});
export const checkoutSchema = Type.Intersect([
	Type.Omit(insertOrderSchema, ["id", "placedAt", "total"]),
	Type.Object({
		items: Type.Array(Type.Omit(insertItemSchema, ["orderId"]), {
			minItems: 1,
		}),
		approval: Type.Union([
			Type.Object({ kind: Type.Literal("automatic"), limit: Type.Number() }),
			Type.Object({
				kind: Type.Literal("manual"),
				reviewers: Type.Array(Type.String({ format: "uuid" })),
				note: Type.Optional(Type.String()),
			}),
		]),
	}),
]);
export const selectOrderSchema = createSelectSchema(orders);
export const statusSchema = createSelectSchema(orderStatus);
export const orderPatchSchema = Type.Intersect([
	Type.Pick(selectOrderSchema, ["id"]),
	Type.Partial(
		Type.Omit(updateOrderSchema, ["id", "organizationId", "buyerId"]),
	),
]);
export const searchSchema = Type.Object({
	organizationId: Type.String({ format: "uuid" }),
	statuses: Type.Array(statusSchema),
	minimum: Type.Optional(Type.Number()),
	tags: Type.Array(Type.String()),
	pagination: Type.Object({
		limit: Type.Integer({ minimum: 1, maximum: 100 }),
		offset: Type.Integer({ minimum: 0 }),
	}),
});
export type Checkout = Static<typeof checkoutSchema>;
export type OrderPatch = Static<typeof orderPatchSchema>;
export type Search = Static<typeof searchSchema>;
export type Product = Static<typeof productSchema>;
export type Order = typeof orders.$inferSelect;
export type NewOrder = typeof orders.$inferInsert;
