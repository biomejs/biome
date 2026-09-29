import * as z from "./vendor/zod";

export const Id = z.uuid().brand<"Id">();
export const OrgSlug = z
	.string()
	.min(3)
	.max(40)
	.regex(/^[a-z0-9-]+$/)
	.transform((slug) => slug.toLowerCase())
	.brand<"OrgSlug">();
export const Timestamp = z.iso.datetime().transform((value) => new Date(value));
export const Email = z.email().transform((value) => value.trim().toLowerCase());

export const Role = z.enum(["owner", "admin", "member", "billing", "viewer"]);
export const Plan = z.enum(["free", "starter", "team", "enterprise"]);
export const Currency = z.enum(["USD", "EUR", "GBP", "JPY"]);

export const roleRank = {
	viewer: 0,
	billing: 1,
	member: 2,
	admin: 3,
	owner: 4,
} as const satisfies Record<z.infer<typeof Role>, number>;

export const Money = z.object({
	amountMinor: z.int().nonnegative(),
	currency: Currency,
});

export const Pagination = z.object({
	cursor: z.string().nullish(),
	limit: z.coerce.number().int().min(1).max(100).default(25),
});

export const SortOrder = z.enum(["asc", "desc"]).default("desc");

export const DateRange = z
	.object({
		from: z.coerce.date(),
		to: z.coerce.date(),
	})
	.refine((range) => range.from <= range.to, {
		message: "`from` must be before `to`",
		path: ["to"],
	});

export const User = z.object({
	id: Id,
	email: Email,
	name: z.string().min(1).max(120),
	avatarUrl: z.url().nullable(),
	locale: z.string().default("en-US"),
	createdAt: Timestamp,
	twoFactorEnabled: z.boolean(),
});

export const PublicUser = User.pick({ id: true, name: true, avatarUrl: true });

export const Session = z.object({
	id: z.string(),
	userId: Id,
	expiresAt: Timestamp,
	impersonatorId: Id.optional(),
});

export const Org = z.object({
	id: Id,
	slug: OrgSlug,
	name: z.string().min(2).max(80),
	plan: Plan,
	seats: z.int().positive(),
	createdAt: Timestamp,
	settings: z.object({
		allowedDomains: z.array(z.string()).default([]),
		requireTwoFactor: z.boolean().default(false),
		defaultRole: Role.exclude(["owner"]).default("member"),
	}),
});

export const Membership = z.object({
	orgId: Id,
	userId: Id,
	role: Role,
	joinedAt: Timestamp,
});

export const Member = Membership.extend({
	user: PublicUser,
	lastActiveAt: Timestamp.nullable(),
});

export const InviteInput = z
	.object({
		email: Email,
		role: Role.exclude(["owner"]),
		message: z.string().max(500).optional(),
		expiresInDays: z.coerce.number().int().min(1).max(30).default(7),
	})
	.strict();

export const Invitation = z.object({
	id: Id,
	orgId: Id,
	email: z.string(),
	role: Role,
	invitedBy: PublicUser,
	expiresAt: Timestamp,
	status: z.enum(["pending", "accepted", "revoked", "expired"]),
});

export const UpdateMemberRoleInput = z
	.object({
		userId: Id,
		role: Role,
		reason: z.string().optional(),
	})
	.superRefine((input, ctx) => {
		if (input.role === "owner" && !input.reason) {
			ctx.addIssue({
				code: "custom",
				message: "Transferring ownership requires a reason",
				path: ["reason"],
			});
		}
	});

export const ProjectStatus = z.enum(["active", "paused", "archived"]);
export const Visibility = z.enum(["private", "org", "public"]);

export const Project = z.object({
	id: Id,
	orgId: Id,
	key: z
		.string()
		.regex(/^[A-Z]{2,6}$/)
		.transform((key) => key as `${Uppercase<string>}`),
	name: z.string().min(1).max(120),
	description: z.string().max(2000).nullable(),
	status: ProjectStatus,
	visibility: Visibility,
	ownerId: Id,
	tags: z.array(z.string().min(1).max(24)).max(10),
	createdAt: Timestamp,
	updatedAt: Timestamp,
	stats: z.object({
		openTasks: z.int().nonnegative(),
		closedTasks: z.int().nonnegative(),
		members: z.int().nonnegative(),
	}),
});

export const CreateProjectInput = Project.pick({
	name: true,
	description: true,
	visibility: true,
	tags: true,
}).extend({
	key: z.string().regex(/^[A-Z]{2,6}$/),
	template: z.enum(["blank", "kanban", "scrum"]).default("blank"),
});

export const UpdateProjectInput = CreateProjectInput.omit({ template: true })
	.partial()
	.extend({ id: Id, status: ProjectStatus.optional() })
	.refine((patch) => Object.keys(patch).length > 1, {
		message: "Nothing to update",
	});

export const ProjectFilter = Pagination.extend({
	status: z.array(ProjectStatus).optional(),
	visibility: Visibility.optional(),
	search: z.string().trim().min(1).optional(),
	tags: z
		.union([z.string(), z.array(z.string())])
		.transform((tags) => (Array.isArray(tags) ? tags : [tags]))
		.optional(),
	sort: z
		.object({
			field: z.enum(["name", "createdAt", "updatedAt", "openTasks"]),
			order: SortOrder,
		})
		.default({ field: "updatedAt", order: "desc" }),
});

export const Priority = z.union([
	z.literal(0),
	z.literal(1),
	z.literal(2),
	z.literal(3),
]);

export const Task = z.object({
	id: Id,
	projectId: Id,
	number: z.int().positive(),
	title: z.string().min(1).max(200),
	body: z.string().max(20_000).default(""),
	priority: Priority,
	assigneeId: Id.nullable(),
	labels: z.array(z.string()),
	dueAt: Timestamp.nullable(),
	state: z.discriminatedUnion("kind", [
		z.object({ kind: z.literal("todo") }),
		z.object({ kind: z.literal("in_progress"), startedAt: Timestamp }),
		z.object({
			kind: z.literal("blocked"),
			blockedBy: z.array(Id).min(1),
			note: z.string().optional(),
		}),
		z.object({
			kind: z.literal("done"),
			completedAt: Timestamp,
			resolution: z.enum(["fixed", "wontfix", "duplicate"]),
		}),
	]),
});

export const TaskCommand = z.discriminatedUnion("type", [
	z.object({
		type: z.literal("assign"),
		taskId: Id,
		assigneeId: Id.nullable(),
	}),
	z.object({
		type: z.literal("move"),
		taskId: Id,
		to: z.enum(["todo", "in_progress", "done"]),
	}),
	z.object({
		type: z.literal("block"),
		taskId: Id,
		blockedBy: z.array(Id).min(1),
		note: z.string().max(280).optional(),
	}),
	z.object({
		type: z.literal("relabel"),
		taskId: Id,
		add: z.array(z.string()).default([]),
		remove: z.array(z.string()).default([]),
	}),
]);

export const InvoiceLine = z.object({
	description: z.string(),
	quantity: z.int().positive(),
	unit: Money,
	period: DateRange.optional(),
});

export const Invoice = z.object({
	id: Id,
	orgId: Id,
	number: z.string().regex(/^INV-\d{6}$/),
	status: z.enum(["draft", "open", "paid", "void", "uncollectible"]),
	issuedAt: Timestamp,
	dueAt: Timestamp,
	lines: z.array(InvoiceLine).min(1),
	subtotal: Money,
	tax: Money,
	total: Money,
});

export const InvoiceSummary = Invoice.pick({
	id: true,
	number: true,
	status: true,
	issuedAt: true,
	total: true,
});

export const PaymentMethod = z.discriminatedUnion("type", [
	z.object({
		type: z.literal("card"),
		brand: z.enum(["visa", "mastercard", "amex"]),
		last4: z.string().length(4),
		expMonth: z.int().min(1).max(12),
		expYear: z.int().min(2024),
	}),
	z.object({
		type: z.literal("sepa"),
		iban: z.string().transform((iban) => `****${iban.slice(-4)}`),
		mandateId: z.string(),
	}),
	z.object({
		type: z.literal("invoice"),
		billingEmail: Email,
		netDays: z.union([z.literal(15), z.literal(30), z.literal(60)]),
	}),
]);

export const ChangePlanInput = z.object({
	plan: Plan.exclude(["free"]),
	seats: z.coerce.number().int().min(1).max(10_000),
	interval: z.enum(["month", "year"]),
	coupon: z
		.string()
		.toUpperCase()
		.regex(/^[A-Z0-9]{4,16}$/)
		.optional(),
});

export const UsageMetric = z.enum([
	"api_calls",
	"storage_gb",
	"seats",
	"builds",
]);

export const UsagePoint = z.object({
	metric: UsageMetric,
	at: Timestamp,
	value: z.number().nonnegative(),
});

export const UsageQuery = z.object({
	metrics: z.array(UsageMetric).min(1),
	range: DateRange,
	granularity: z.enum(["hour", "day", "week"]).default("day"),
});

export const ApiKeyScope = z.enum([
	"projects:read",
	"projects:write",
	"tasks:read",
	"tasks:write",
	"billing:read",
]);

export const ApiKey = z.object({
	id: Id,
	name: z.string().min(1).max(60),
	prefix: z.string().length(8),
	scopes: z.array(ApiKeyScope).min(1),
	createdAt: Timestamp,
	lastUsedAt: Timestamp.nullable(),
	expiresAt: Timestamp.nullable(),
});

export const CreateApiKeyInput = ApiKey.pick({
	name: true,
	scopes: true,
}).extend({
	expiresInDays: z.coerce.number().int().positive().max(365).optional(),
});

export const WebhookEvent = z.enum([
	"project.created",
	"project.archived",
	"task.updated",
	"member.joined",
	"invoice.paid",
]);

export const Webhook = z.object({
	id: Id,
	url: z.url(),
	events: z.array(WebhookEvent).min(1),
	secret: z.string().min(16),
	active: z.boolean(),
});

export const AuditEvent = z.discriminatedUnion("action", [
	z.object({
		action: z.literal("member.invited"),
		email: z.string(),
		role: Role,
	}),
	z.object({
		action: z.literal("member.role_changed"),
		userId: Id,
		from: Role,
		to: Role,
	}),
	z.object({
		action: z.literal("project.deleted"),
		projectId: Id,
		name: z.string(),
	}),
	z.object({
		action: z.literal("billing.plan_changed"),
		from: Plan,
		to: Plan,
		seats: z.int(),
	}),
	z.object({
		action: z.literal("apikey.created"),
		keyId: Id,
		scopes: z.array(ApiKeyScope),
	}),
]);

export const AuditEntry = z.object({
	id: Id,
	at: Timestamp,
	actor: PublicUser,
	ip: z.string().nullable(),
	event: AuditEvent,
});

export const Notification = z.object({
	id: Id,
	at: Timestamp,
	read: z.boolean(),
	payload: z.discriminatedUnion("kind", [
		z.object({ kind: z.literal("mention"), taskId: Id, by: PublicUser }),
		z.object({ kind: z.literal("invite"), orgName: z.string() }),
		z.object({
			kind: z.literal("billing"),
			severity: z.enum(["info", "warning", "critical"]),
			message: z.string(),
		}),
	]),
});

export function page<T extends z.ZodType>(item: T) {
	return z.object({
		items: z.array(item),
		nextCursor: z.string().nullable(),
		total: z.int().nonnegative(),
	});
}

export type Id = z.infer<typeof Id>;
export type Role = z.infer<typeof Role>;
export type Plan = z.infer<typeof Plan>;
export type Money = z.infer<typeof Money>;
export type User = z.infer<typeof User>;
export type PublicUser = z.infer<typeof PublicUser>;
export type Org = z.infer<typeof Org>;
export type Membership = z.infer<typeof Membership>;
export type Member = z.infer<typeof Member>;
export type Project = z.infer<typeof Project>;
export type ProjectFilterInput = z.input<typeof ProjectFilter>;
export type ProjectFilter = z.output<typeof ProjectFilter>;
export type Task = z.infer<typeof Task>;
export type TaskState = Task["state"];
export type TaskCommand = z.infer<typeof TaskCommand>;
export type Invoice = z.infer<typeof Invoice>;
export type PaymentMethod = z.infer<typeof PaymentMethod>;
export type AuditEvent = z.infer<typeof AuditEvent>;
export type Notification = z.infer<typeof Notification>;
export type ApiKeyScope = z.infer<typeof ApiKeyScope>;
