import type {
	ColumnType,
	Generated,
	GeneratedAlways,
	Insertable,
	JSONColumnType,
	Kysely,
	Selectable,
	Updateable,
} from "./vendor/kysely";

export type Timestamp = ColumnType<Date, Date | string | undefined, never>;
export type MutableTimestamp = ColumnType<Date, Date | string, Date | string>;
export type Cents = ColumnType<number, number | bigint, number | bigint>;
export type Currency = "USD" | "EUR" | "GBP" | "JPY" | "CAD";
export type CountryCode = "US" | "CA" | "GB" | "DE" | "FR" | "JP" | "NL";

export type UserRole = "buyer" | "seller" | "admin" | "support";
export type OrderStatus =
	| "pending"
	| "paid"
	| "packed"
	| "shipped"
	| "delivered"
	| "cancelled"
	| "refunded";
export type PaymentProvider = "stripe" | "paypal" | "adyen";
export type PaymentStatus = "authorized" | "captured" | "failed" | "voided";
export type ProductStatus = "draft" | "active" | "archived";
export type CouponKind = "percent" | "fixed" | "shipping";

export interface UserMetadata {
	readonly marketingOptIn: boolean;
	readonly locale: `${Lowercase<string>}-${Uppercase<string>}`;
	readonly acquisition:
		| { readonly channel: "organic" }
		| { readonly channel: "referral"; readonly code: string }
		| {
				readonly channel: "paid";
				readonly campaign: string;
				readonly costCents: number;
		  };
	readonly flags: ReadonlyArray<"beta" | "vip" | "fraud_review" | "wholesale">;
}

export interface StoreSettings {
	readonly payoutSchedule: "daily" | "weekly" | "monthly";
	readonly vacation: { readonly from: string; readonly until: string } | null;
	readonly shipping: ReadonlyArray<{
		readonly region: CountryCode;
		readonly flatRateCents: number;
		readonly freeOverCents?: number;
	}>;
}

export interface Address {
	readonly line1: string;
	readonly line2?: string;
	readonly city: string;
	readonly postalCode: string;
	readonly country: CountryCode;
}

export type ProductAttributes = Record<string, string | number | boolean>;

export type PaymentPayload =
	| {
			readonly provider: "stripe";
			readonly chargeId: `ch_${string}`;
			readonly outcome: {
				readonly riskLevel: "normal" | "elevated" | "highest";
			};
	  }
	| {
			readonly provider: "paypal";
			readonly captureId: string;
			readonly payerEmail: string;
	  }
	| {
			readonly provider: "adyen";
			readonly pspReference: string;
			readonly resultCode: "Authorised" | "Refused" | "Error" | "Cancelled";
			readonly refusalReason?: string;
	  };

export type EventPayload =
	| {
			readonly type: "order.placed";
			readonly orderId: string;
			readonly totalCents: number;
			readonly currency: Currency;
	  }
	| {
			readonly type: "order.status_changed";
			readonly orderId: string;
			readonly from: OrderStatus;
			readonly to: OrderStatus;
	  }
	| {
			readonly type: "payment.failed";
			readonly paymentId: string;
			readonly code: string;
			readonly retryable: boolean;
	  }
	| {
			readonly type: "inventory.adjusted";
			readonly productId: string;
			readonly warehouse: string;
			readonly delta: number;
	  }
	| {
			readonly type: "review.flagged";
			readonly reviewId: number;
			readonly reasons: ReadonlyArray<"spam" | "abuse" | "off_topic">;
	  };

export interface UsersTable {
	id: Generated<string>;
	email: string;
	display_name: string;
	role: UserRole;
	country_code: CountryCode;
	referred_by: string | null;
	metadata: JSONColumnType<UserMetadata>;
	created_at: Timestamp;
	last_seen_at: Date | null;
}

export interface StoresTable {
	id: Generated<string>;
	owner_id: string;
	name: string;
	slug: string;
	rating: number | null;
	settings: JSONColumnType<StoreSettings>;
	created_at: Timestamp;
}

export interface CategoriesTable {
	id: Generated<number>;
	parent_id: number | null;
	name: string;
	slug: string;
	position: number;
}

export interface ProductsTable {
	id: Generated<string>;
	store_id: string;
	category_id: number | null;
	title: string;
	sku: string;
	price_cents: Cents;
	currency: Currency;
	status: ProductStatus;
	attributes: JSONColumnType<ProductAttributes>;
	created_at: Timestamp;
	updated_at: MutableTimestamp;
}

export interface WarehousesTable {
	id: Generated<number>;
	code: string;
	country_code: CountryCode;
}

export interface InventoryTable {
	product_id: string;
	warehouse_id: number;
	quantity: number;
	reserved: Generated<number>;
	updated_at: MutableTimestamp;
}

export interface CouponsTable {
	id: Generated<number>;
	code: string;
	kind: CouponKind;
	value: number;
	store_id: string | null;
	expires_at: Date | null;
}

export interface OrdersTable {
	id: Generated<string>;
	buyer_id: string;
	seller_id: string;
	store_id: string;
	coupon_id: number | null;
	status: OrderStatus;
	total_cents: Cents;
	currency: Currency;
	shipping_address: JSONColumnType<Address>;
	placed_at: Timestamp;
	fulfilled_at: Date | null;
	version: Generated<number>;
}

export interface OrderItemsTable {
	id: GeneratedAlways<number>;
	order_id: string;
	product_id: string;
	quantity: number;
	unit_price_cents: Cents;
	discount_cents: Generated<number>;
}

export interface PaymentsTable {
	id: Generated<string>;
	order_id: string;
	provider: PaymentProvider;
	status: PaymentStatus;
	amount_cents: Cents;
	failure_code: string | null;
	payload: JSONColumnType<PaymentPayload>;
	created_at: Timestamp;
}

export interface RefundsTable {
	id: Generated<string>;
	payment_id: string;
	amount_cents: Cents;
	reason: "damaged" | "not_received" | "fraud" | "goodwill";
	approved_by: string | null;
	created_at: Timestamp;
}

export interface ReviewsTable {
	id: Generated<number>;
	product_id: string;
	author_id: string;
	order_id: string | null;
	rating: 1 | 2 | 3 | 4 | 5;
	body: string | null;
	created_at: Timestamp;
}

export interface EventsTable {
	id: GeneratedAlways<string>;
	aggregate_type: "order" | "payment" | "product" | "review";
	aggregate_id: string;
	payload: JSONColumnType<EventPayload>;
	occurred_at: Timestamp;
	processed_at: Date | null;
}

export interface DailySalesTable {
	store_id: string;
	day: string;
	gross_cents: Cents;
	refunded_cents: Cents;
	order_count: number;
	updated_at: MutableTimestamp;
}

export interface Database {
	users: UsersTable;
	stores: StoresTable;
	categories: CategoriesTable;
	products: ProductsTable;
	warehouses: WarehousesTable;
	inventory: InventoryTable;
	coupons: CouponsTable;
	orders: OrdersTable;
	order_items: OrderItemsTable;
	payments: PaymentsTable;
	refunds: RefundsTable;
	reviews: ReviewsTable;
	events: EventsTable;
	daily_sales: DailySalesTable;
}

export type User = Selectable<UsersTable>;
export type NewUser = Insertable<UsersTable>;
export type UserUpdate = Updateable<UsersTable>;
export type Store = Selectable<StoresTable>;
export type Category = Selectable<CategoriesTable>;
export type Product = Selectable<ProductsTable>;
export type NewProduct = Insertable<ProductsTable>;
export type ProductUpdate = Updateable<ProductsTable>;
export type Order = Selectable<OrdersTable>;
export type NewOrder = Insertable<OrdersTable>;
export type OrderUpdate = Updateable<OrdersTable>;
export type OrderItem = Selectable<OrderItemsTable>;
export type NewOrderItem = Insertable<OrderItemsTable>;
export type Payment = Selectable<PaymentsTable>;
export type Refund = Selectable<RefundsTable>;
export type Review = Selectable<ReviewsTable>;
export type DomainEvent = Selectable<EventsTable>;
export type NewDomainEvent = Insertable<EventsTable>;
export type DailySales = Selectable<DailySalesTable>;
export type NewDailySales = Insertable<DailySalesTable>;

export declare const db: Kysely<Database>;
