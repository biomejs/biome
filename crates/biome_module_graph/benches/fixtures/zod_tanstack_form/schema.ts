import { z } from "./vendor/zod";

const identifier = z.string().uuid().brand<"Identifier">();
const money = z.object({
	amount: z.number().nonnegative(),
	currency: z.enum(["USD", "EUR", "GBP", "JPY"]),
});
const address = z.object({
	lines: z.tuple([z.string(), z.string().optional()]),
	city: z.string().min(1),
	region: z.string().optional(),
	postalCode: z.string(),
	country: z.enum(["US", "GB", "DE", "JP"]),
	coordinates: z.tuple([z.number(), z.number()]).nullable(),
});
const contact = z.object({
	id: identifier,
	name: z.object({
		first: z.string(),
		last: z.string(),
		preferred: z.string().optional(),
	}),
	email: z.string().email(),
	addresses: z.array(
		address.extend({ label: z.enum(["home", "work", "billing"]) }),
	),
	notifications: z.record(
		z.enum(["email", "sms", "push"]),
		z.object({
			enabled: z.boolean(),
			schedule: z.object({
				days: z.array(z.number().int().min(0).max(6)),
				timezone: z.string(),
			}),
		}),
	),
});
const inventory = z.object({
	sku: z.string(),
	quantity: z.number().int().positive(),
	price: money,
	discounts: z.array(
		z.discriminatedUnion("kind", [
			z.object({
				kind: z.literal("percentage"),
				percent: z.number().min(0).max(100),
			}),
			z.object({ kind: z.literal("fixed"), value: money }),
			z.object({
				kind: z.literal("tiered"),
				tiers: z.array(z.object({ minimum: z.number(), value: money })),
			}),
		]),
	),
	attributes: z.record(
		z.union([z.string(), z.number(), z.boolean(), z.array(z.string())]),
	),
});
const physicalItem = inventory.extend({
	kind: z.literal("physical"),
	shipping: z.object({
		dimensions: z.tuple([z.number(), z.number(), z.number()]),
		weight: z.number(),
		warehouse: address,
		restrictions: z.array(z.enum(["fragile", "hazardous", "refrigerated"])),
	}),
});
const digitalItem = inventory.extend({
	kind: z.literal("digital"),
	license: z.object({
		seats: z.number(),
		expiresAt: z.string().datetime().nullable(),
	}),
	downloads: z.array(
		z.object({
			url: z.string().url(),
			checksum: z.string(),
			formats: z.array(z.string()),
		}),
	),
});
const subscriptionItem = inventory.extend({
	kind: z.literal("subscription"),
	billing: z.object({
		interval: z.enum(["month", "year"]),
		trialDays: z.number(),
		renewal: money,
	}),
	entitlements: z.record(
		z.object({ limit: z.number().nullable(), features: z.array(z.string()) }),
	),
});
const item = z.discriminatedUnion("kind", [
	physicalItem,
	digitalItem,
	subscriptionItem,
]);
const payment = z.discriminatedUnion("method", [
	z.object({
		method: z.literal("card"),
		token: z.string(),
		billingAddress: address,
		installments: z.number(),
	}),
	z.object({
		method: z.literal("bank"),
		iban: z.string(),
		accountHolder: contact.pick({ name: true, email: true }),
	}),
	z.object({
		method: z.literal("invoice"),
		purchaseOrder: z.string(),
		approvers: z.array(contact),
		dueDays: z.number(),
	}),
]);
const shipment = z.object({
	destination: address,
	items: z.array(z.object({ sku: z.string(), quantity: z.number() })),
	service: z.enum(["economy", "express", "overnight"]),
	tracking: z.array(
		z.object({
			timestamp: z.string().datetime(),
			location: address.partial(),
			status: z.string(),
		}),
	),
});

export const checkoutSchema = z
	.object({
		customer: contact.extend({
			organization: z
				.object({
					name: z.string(),
					taxId: z.string().optional(),
					departments: z.array(z.string()),
				})
				.optional(),
			preferences: z.object({
				locale: z.string(),
				currency: money.shape.currency,
				tags: z.array(z.string()),
			}),
		}),
		orders: z.array(
			z.object({
				id: identifier,
				items: z.array(item).min(1),
				payment,
				shipments: z.array(shipment),
				adjustments: z.record(money),
				notes: z.array(
					z.object({
						author: contact.pick({ id: true, name: true }),
						text: z.string(),
						visible: z.boolean(),
					}),
				),
			}),
		),
		delivery: z.object({
			primary: address,
			fallback: address.nullable(),
			instructions: z.string().optional(),
		}),
		approvals: z.array(
			z.object({
				reviewer: contact,
				scope: z.array(identifier),
				decision: z.enum(["pending", "approved", "rejected"]),
			}),
		),
		metadata: z.record(
			z.object({
				source: z.string(),
				values: z.array(z.union([z.string(), z.number(), z.boolean()])),
			}),
		),
		consent: z.object({
			terms: z.boolean(),
			marketing: z.boolean(),
			version: z.string(),
		}),
	})
	.strict();

export const submissionSchema = checkoutSchema
	.superRefine((value, context) => {
		for (const [orderIndex, order] of value.orders.entries()) {
			const total = order.items.reduce(
				(sum, entry) => sum + entry.price.amount * entry.quantity,
				0,
			);
			if (
				order.payment.method === "invoice" &&
				total > 10000 &&
				order.payment.approvers.length < 2
			) {
				context.addIssue({
					code: "custom",
					path: ["orders", orderIndex, "payment", "approvers"],
					message: "Two approvers required",
				});
			}
		}
	})
	.transform((value) => ({
		...value,
		customer: { ...value.customer, email: value.customer.email.toLowerCase() },
		totals: value.orders.map((order) => ({
			orderId: order.id,
			total: order.items.reduce(
				(sum, entry) => sum + entry.price.amount * entry.quantity,
				0,
			),
			physicalSkus: order.items
				.filter((entry) => entry.kind === "physical")
				.map((entry) => entry.sku),
		})),
	}));

export const draftSchema = checkoutSchema
	.omit({ approvals: true })
	.deepPartial()
	.extend({
		revision: z.number().int(),
		restoredFrom: identifier.nullable(),
	});
export const summarySchema = checkoutSchema
	.pick({ customer: true, consent: true })
	.merge(
		z.object({
			totals: z.array(money),
			status: z.enum(["draft", "submitted", "fulfilled"]),
		}),
	);
export type Checkout = z.input<typeof checkoutSchema>;
export type Submission = z.output<typeof submissionSchema>;
export type Draft = z.infer<typeof draftSchema>;
export type Summary = z.infer<typeof summarySchema>;

export function parseSubmission(input: unknown) {
	const result = submissionSchema.safeParse(input);
	return result.success
		? {
				status: "accepted" as const,
				totals: result.data.totals,
				customer: result.data.customer,
			}
		: {
				status: "rejected" as const,
				errors: result.error.flatten(),
				issues: result.error.format(),
			};
}

export const parsedDraft = draftSchema.parse({
	revision: 1,
	restoredFrom: null,
});
export const parsedSubmission = submissionSchema.parse({});
export const parsedSummary = summarySchema.safeParse({});
