import { type, scope, match, type Type } from "./vendor/arktype";

export const Money = type({
	amount: "number.integer >= 0",
	currency: "'USD' | 'EUR' | 'GBP' | 'JPY'",
});

export const Address = type({
	line1: "string > 0",
	"line2?": "string",
	city: "string > 0",
	region: "string",
	postalCode: /^[A-Z0-9 -]{3,10}$/,
	country: "'US' | 'CA' | 'GB' | 'DE' | 'FR' | 'JP'",
});

export const Coordinates = type(["number >= -90", "number <= 180"]).narrow(
	([lat]) => lat <= 90,
);

const catalog = scope({
	Sku: /^[A-Z]{3}-\d{4,6}$/,
	Dimensions: {
		lengthCm: "0 < number <= 400",
		widthCm: "0 < number <= 400",
		heightCm: "0 < number <= 400",
		weightKg: "0 < number <= 1000",
	},
	Variant: {
		sku: "Sku",
		attributes: { "[string]": "string | number | boolean" },
		price: Money,
		"compareAt?": Money,
		dimensions: "Dimensions",
		stock: "number.integer >= 0",
		"tags?": "string[]",
	},
	Category: {
		id: "string.uuid",
		name: "string > 0",
		children: "Category[]",
		"parent?": "string.uuid | null",
	},
	Product: {
		id: "string.uuid",
		title: "1 <= string <= 120",
		kind: "'physical' | 'digital' | 'service'",
		categories: "Category[]",
		variants: "Variant[] > 0",
		"bundle?": "BundleItem[]",
		metadata: "Record<string, string>",
	},
	BundleItem: {
		sku: "Sku",
		quantity: "1 <= number.integer <= 99",
		"substitutes?": "Sku[]",
	},
}).export();

export const Sku = catalog.Sku;
export const Product = catalog.Product;
export const Variant = catalog.Variant;
export const Category = catalog.Category;

export const LineItem = type({
	sku: Sku,
	quantity: "1 <= number.integer <= 999",
	unitPrice: Money,
	"discounts?": type({
		code: "string",
		kind: "'percent' | 'fixed'",
		value: "number > 0",
	}).array(),
	"giftWrap?": { message: "string <= 240", style: "'classic' | 'festive'" },
});

export const PaymentMethod = type({
	kind: "'card'",
	brand: "'visa' | 'mastercard' | 'amex'",
	last4: /^\d{4}$/,
	expires: {
		month: "1 <= number.integer <= 12",
		year: "number.integer >= 2024",
	},
})
	.or({
		kind: "'wallet'",
		provider: "'apple' | 'google' | 'paypal'",
		token: "string",
	})
	.or({
		kind: "'invoice'",
		terms: "'net15' | 'net30' | 'net60'",
		purchaseOrder: "string | null",
	});

export const Customer = type({
	id: "string.uuid",
	email: "string.email",
	name: { given: "string", family: "string", "preferred?": "string" },
	tier: "'standard' | 'silver' | 'gold' | 'platinum'",
	addresses: {
		billing: Address,
		shipping: Address.array().atLeastLength(1),
	},
	"locale?": "'en-US' | 'en-GB' | 'de-DE' | 'fr-FR' | 'ja-JP'",
	createdAt: "string.date.iso.parse",
	flags: type("string")
		.array()
		.pipe((flags) => new Set(flags)),
});

export const OrderDraft = type({
	customer: Customer,
	items: LineItem.array().moreThanLength(0),
	payment: PaymentMethod,
	shippingMethod: "'ground' | 'express' | 'overnight' | 'pickup'",
	"pickupLocation?": Coordinates,
	"notes?": "string <= 2000",
	"coupon?": /^[A-Z0-9]{6,12}$/,
	requestedAt: "string.date.iso.parse",
	channel: "'web' | 'mobile' | 'pos' | 'marketplace'",
});

export const Order = OrderDraft.merge({
	id: "string.uuid",
	status:
		"'pending' | 'authorized' | 'picking' | 'packed' | 'shipped' | 'delivered' | 'cancelled'",
	totals: {
		subtotal: Money,
		tax: Money,
		shipping: Money,
		discount: Money,
		grand: Money,
	},
	revision: "number.integer >= 0",
});

export const OrderPatch = Order.pick("status", "notes", "shippingMethod")
	.partial()
	.and({ revision: "number.integer >= 0" });

export const OrderSummary = Order.omit("customer", "payment", "items").and({
	itemCount: "number.integer",
	customerEmail: "string.email",
});

const events = scope({
	Base: {
		eventId: "string.uuid",
		orderId: "string.uuid",
		at: "string.date.iso.parse",
		version: "number.integer >= 1",
	},
	OrderPlaced: {
		"...": "Base",
		type: "'order.placed'",
		payload: OrderDraft,
	},
	PaymentAuthorized: {
		"...": "Base",
		type: "'payment.authorized'",
		payload: {
			authorizationId: "string",
			amount: Money,
			"risk?": "0 <= number <= 1",
		},
	},
	PaymentDeclined: {
		"...": "Base",
		type: "'payment.declined'",
		payload: {
			reason: "'insufficient_funds' | 'fraud' | 'expired' | 'unknown'",
			retryable: "boolean",
		},
	},
	ItemsPicked: {
		"...": "Base",
		type: "'items.picked'",
		payload: {
			warehouse: "string",
			picks: [
				{ sku: Sku, bin: "string", quantity: "number.integer > 0" },
				"[]",
			],
		},
	},
	ShipmentCreated: {
		"...": "Base",
		type: "'shipment.created'",
		payload: {
			carrier: "'ups' | 'fedex' | 'dhl' | 'usps'",
			tracking: "string",
			parcels: ["string", "...", "number[]"],
			"eta?": "string.date.iso.parse",
		},
	},
	OrderCancelled: {
		"...": "Base",
		type: "'order.cancelled'",
		payload: { reason: "string", refund: "Money | null" },
	},
	Money: Money,
	Event:
		"OrderPlaced | PaymentAuthorized | PaymentDeclined | ItemsPicked | ShipmentCreated | OrderCancelled",
	Envelope: {
		stream: "string",
		sequence: "number.integer >= 0",
		events: "Event[]",
		"cursor?": "string | null",
	},
}).export();

export const OrderEvent = events.Event;
export const EventEnvelope = events.Envelope;

export const Page = type("<t>", {
	items: "t[]",
	total: "number.integer >= 0",
	page: "number.integer >= 0",
	size: "1 <= number.integer <= 200",
	"next?": "string | null",
});

export const OrderPage = Page(OrderSummary);
export const ProductPage = Page(Product);

export const SearchQuery = type({
	"q?": "string",
	"status?": Order.get("status").array(),
	"channel?": "('web' | 'mobile' | 'pos' | 'marketplace')[]",
	"minTotal?": "string.numeric.parse",
	"maxTotal?": "string.numeric.parse",
	"placedAfter?": "string.date.iso.parse",
	page: "string.integer.parse = '0'",
	size: "string.integer.parse = '25'",
	sort: "'placedAt' | 'total' | 'status' = 'placedAt'",
	direction: "'asc' | 'desc' = 'desc'",
});

export const WebhookConfig = type({
	url: "string.url",
	secret: "string >= 32",
	events: OrderEvent.get("type").array().atLeastLength(1),
	retry: {
		maxAttempts: "1 <= number.integer <= 10",
		backoff: "'fixed' | 'exponential'",
		baseDelayMs: "number.integer >= 100",
	},
	headers: "Record<string, string>",
});

export const RuntimeConfig = type({
	database: {
		url: "string.url",
		poolSize: "string.integer.parse",
		"ssl?": "'require' | 'prefer' | 'disable'",
	},
	payments: {
		provider: "'stripe' | 'adyen'",
		apiKey: "string >= 16",
		timeoutMs: "string.integer.parse",
	},
	warehouse: {
		regions: "string[] > 0",
		cutoffHour: "0 <= number.integer < 24",
	},
	webhooks: WebhookConfig.array(),
	featureFlags: { "[string]": "boolean" },
});

export const describeEvent = match({
	"string.uuid": (id) => `event ${id}`,
	"number.integer": (seq) => `sequence #${seq}`,
	default: "assert",
});

export const eventLabel = match
	.in<typeof OrderEvent.infer>()
	.at("type")
	.match({
		"'order.placed'": (e) => `${e.payload.customer.email} placed ${e.orderId}`,
		"'payment.authorized'": (e) => `authorized ${e.payload.amount.amount}`,
		"'payment.declined'": (e) => `declined: ${e.payload.reason}`,
		"'items.picked'": (e) =>
			`${e.payload.picks.length} picks in ${e.payload.warehouse}`,
		"'shipment.created'": (e) => `${e.payload.carrier} ${e.payload.tracking}`,
		"'order.cancelled'": (e) => `cancelled: ${e.payload.reason}`,
		default: "never",
	});

export type Money = typeof Money.infer;
export type Address = typeof Address.infer;
export type Product = typeof Product.infer;
export type Variant = typeof Variant.infer;
export type Category = typeof Category.infer;
export type LineItem = typeof LineItem.infer;
export type PaymentMethod = typeof PaymentMethod.infer;
export type Customer = typeof Customer.infer;
export type CustomerInput = typeof Customer.inferIn;
export type OrderDraft = typeof OrderDraft.infer;
export type Order = typeof Order.infer;
export type OrderPatch = typeof OrderPatch.infer;
export type OrderSummary = typeof OrderSummary.infer;
export type OrderEvent = typeof OrderEvent.infer;
export type EventEnvelope = typeof EventEnvelope.infer;
export type OrderPage = typeof OrderPage.infer;
export type SearchQuery = typeof SearchQuery.infer;
export type SearchQueryInput = typeof SearchQuery.inferIn;
export type WebhookConfig = typeof WebhookConfig.infer;
export type RuntimeConfig = typeof RuntimeConfig.infer;
export type AnySchema = Type<unknown, {}>;
