import {
	FieldApi,
	FormApi,
	formOptions,
	type DeepKeys,
	type DeepValue,
} from "./vendor/tanstack-form";
import {
	checkoutSchema,
	submissionSchema,
	type Checkout,
	type Submission,
} from "./schema";

const defaults: Checkout = {
	customer: {
		id: "00000000-0000-0000-0000-000000000001" as Checkout["customer"]["id"],
		name: { first: "Ada", last: "Lovelace" },
		email: "ada@example.com",
		addresses: [],
		notifications: {},
		preferences: { locale: "en-GB", currency: "GBP", tags: [] },
	},
	orders: [],
	delivery: {
		primary: {
			lines: ["1 Example Road", undefined],
			city: "London",
			postalCode: "SW1A",
			country: "GB",
			coordinates: null,
		},
		fallback: null,
	},
	approvals: [],
	metadata: {},
	consent: { terms: false, marketing: false, version: "2025-01" },
};

export const checkoutOptions = formOptions({
	defaultValues: defaults,
	validators: {
		onChange: checkoutSchema,
		onBlur: ({ value }) =>
			value.orders.length === 0 ? "Add an order" : undefined,
		onSubmitAsync: async ({ value }) => {
			const result = await submissionSchema.safeParseAsync(value);
			return result.success
				? undefined
				: {
						form: result.error.message,
						fields: { "customer.email": "Review customer details" },
					};
		},
	},
	onSubmitMeta: {
		source: "checkout" as const,
		audit: { actor: "customer", retry: 0 },
	},
	onSubmit: async ({ value, formApi, meta }) => {
		const submitted = await submissionSchema.parseAsync(value);
		formApi.setFieldValue("metadata", (previous) => ({
			...previous,
			submission: {
				source: meta.source,
				values: submitted.totals.map((total) => total.total),
			},
		}));
	},
});
export const checkoutForm = new FormApi(checkoutOptions);

export const emailField = new FieldApi({
	form: checkoutForm,
	name: "customer.email",
	validators: {
		onChange: checkoutSchema.shape.customer.shape.email,
		onBlurAsync: async ({ value, fieldApi }) => {
			const firstName = fieldApi.form.getFieldValue("customer.name.first");
			return value.includes(firstName.toLowerCase())
				? undefined
				: "Confirm email ownership";
		},
	},
});
export const quantityField = new FieldApi({
	form: checkoutForm,
	name: "orders[0].items[0].quantity",
	validators: {
		onChange: ({ value }) =>
			value > 1000 ? "Quantity requires approval" : undefined,
		onChangeListenTo: ["orders[0].items[0].price.amount"],
		onBlur: ({ value, fieldApi }) => {
			const amount = fieldApi.form.getFieldValue(
				"orders[0].items[0].price.amount",
			);
			return amount * value > 10000 ? "Review purchase limit" : undefined;
		},
	},
});
export const countryField = new FieldApi({
	form: checkoutForm,
	name: "delivery.primary.country",
});
export const consentField = new FieldApi({
	form: checkoutForm,
	name: "consent.terms",
});

export type CheckoutPaths = DeepKeys<Checkout>;
export type FieldValues = {
	[Path in CheckoutPaths]: DeepValue<Checkout, Path>;
};
export type FieldsOfType<Value> = {
	[Path in CheckoutPaths]: DeepValue<Checkout, Path> extends Value
		? Path
		: never;
}[CheckoutPaths];
export type NumericFields = FieldsOfType<number>;
export type StringFields = FieldsOfType<string>;
export type SubmissionPaths = DeepKeys<Submission>;
export type SubmissionValues = {
	[Path in SubmissionPaths]: DeepValue<Submission, Path>;
};

export function readField<Path extends CheckoutPaths>(
	path: Path,
): DeepValue<Checkout, Path> {
	return checkoutForm.getFieldValue(path);
}
export function updateCheckout() {
	checkoutForm.setFieldValue("customer.name.first", (previous) =>
		previous.trim(),
	);
	checkoutForm.setFieldValue(
		"orders[0].items[0].quantity",
		(previous) => previous + 1,
	);
	checkoutForm.setFieldValue(
		"orders[0].items[0].price.amount",
		(previous) => Math.round(previous * 100) / 100,
	);
	checkoutForm.setFieldValue("customer.preferences.tags", (previous) => [
		...previous,
		"priority",
	]);
	checkoutForm.setFieldValue("delivery.primary.coordinates", [51.5, -0.12]);
	checkoutForm.setFieldValue("consent.terms", true);
	checkoutForm.pushFieldValue("customer.addresses", {
		...defaults.delivery.primary,
		label: "billing",
	});
	checkoutForm.pushFieldValue("approvals", {
		reviewer: defaults.customer,
		scope: [],
		decision: "pending",
	});
	emailField.handleChange("ada.lovelace@example.com");
	quantityField.handleChange(4);
	countryField.handleChange("DE");
}

export const emailValue = checkoutForm.getFieldValue("customer.email");
export const quantityValue = checkoutForm.getFieldValue(
	"orders[0].items[0].quantity",
);
export const currencyValue = checkoutForm.getFieldValue(
	"orders[0].items[0].price.currency",
);
export const paymentValue = checkoutForm.getFieldValue("orders[0].payment");
export const coordinatesValue = checkoutForm.getFieldValue(
	"delivery.primary.coordinates",
);
export const notificationValue = checkoutForm.getFieldValue(
	"customer.notifications.email.schedule.days",
);
export const fieldValue = quantityField.state.value;
export const fieldErrors = emailField.state.meta.errors;
export const submission = checkoutForm.handleSubmit({
	source: "checkout",
	audit: { actor: "customer", retry: 1 },
});
