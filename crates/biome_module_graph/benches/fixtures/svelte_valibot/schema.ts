import * as v from "./vendor/valibot";

const identifier = v.pipe(v.string(), v.uuid(), v.brand("Identifier"));
const label = v.pipe(v.string(), v.trim(), v.minLength(1));
const money = v.object({
	amount: v.pipe(v.number(), v.minValue(0)),
	currency: v.picklist(["USD", "EUR", "GBP"]),
});
const address = v.object({
	lines: v.tuple([label, v.optional(v.string())]),
	city: label,
	country: v.picklist(["US", "GB", "DE"]),
	postalCode: label,
	coordinates: v.nullable(v.tuple([v.number(), v.number()])),
});
const member = v.object({
	id: identifier,
	name: label,
	email: v.pipe(v.string(), v.trim(), v.toLowerCase(), v.email()),
	role: v.picklist(["owner", "editor", "viewer"]),
	availability: v.array(
		v.object({
			day: v.number(),
			hours: v.tuple([v.number(), v.number()]),
			timezone: label,
		}),
	),
	contact: v.object({ address, phone: v.optional(v.string()) }),
});
const discount = v.variant("kind", [
	v.object({
		kind: v.literal("percent"),
		percent: v.pipe(v.number(), v.minValue(0), v.maxValue(100)),
	}),
	v.object({ kind: v.literal("fixed"), amount: money }),
	v.object({
		kind: v.literal("volume"),
		tiers: v.array(v.object({ minimum: v.number(), amount: money })),
	}),
]);
const task = v.variant("kind", [
	v.object({
		kind: v.literal("onsite"),
		id: identifier,
		title: label,
		assignees: v.array(member),
		location: address,
		equipment: v.array(
			v.object({
				code: label,
				count: v.pipe(v.number(), v.integer()),
				cost: money,
			}),
		),
		safety: v.object({
			permits: v.array(label),
			supervisor: member,
			reviewed: v.boolean(),
		}),
	}),
	v.object({
		kind: v.literal("remote"),
		id: identifier,
		title: label,
		assignees: v.array(member),
		connection: v.object({
			url: v.pipe(v.string(), v.url()),
			provider: v.picklist(["meet", "zoom", "teams"]),
		}),
		recordings: v.array(
			v.object({
				url: v.string(),
				duration: v.number(),
				transcript: v.optional(v.string()),
			}),
		),
	}),
	v.object({
		kind: v.literal("delivery"),
		id: identifier,
		title: label,
		assignees: v.array(member),
		destination: address,
		tracking: v.array(
			v.object({
				time: v.string(),
				status: label,
				location: v.partial(address),
			}),
		),
		packages: v.array(
			v.object({
				weight: v.number(),
				dimensions: v.tuple([v.number(), v.number(), v.number()]),
				insured: money,
			}),
		),
	}),
]);
const phase = v.object({
	id: identifier,
	title: label,
	tasks: v.array(task),
	budget: money,
	discounts: v.array(discount),
	milestones: v.array(
		v.object({ name: label, due: v.string(), approvedBy: v.nullable(member) }),
	),
	dependencies: v.record(v.string(), v.array(identifier)),
});
export const workspaceSchema = v.strictObject({
	organization: v.object({
		id: identifier,
		name: label,
		billing: address,
		contacts: v.array(member),
	}),
	projects: v.array(
		v.object({
			id: identifier,
			name: label,
			state: v.picklist(["draft", "active", "paused", "complete"]),
			owner: member,
			phases: v.array(phase),
			collaborators: v.array(member),
			settings: v.object({
				timezone: label,
				visibility: v.picklist(["private", "team", "public"]),
				tags: v.array(label),
			}),
			attributes: v.record(
				v.string(),
				v.union([v.string(), v.number(), v.boolean(), v.array(v.string())]),
			),
		}),
	),
	permissions: v.record(
		v.string(),
		v.object({ read: v.boolean(), write: v.boolean(), scopes: v.array(label) }),
	),
	preferences: v.object({
		currency: v.picklist(["USD", "EUR", "GBP"]),
		notifications: v.record(v.string(), v.boolean()),
	}),
	audit: v.array(
		v.object({
			actor: v.pick(member, ["id", "name"]),
			action: label,
			changes: v.record(v.string(), v.tuple([v.string(), v.string()])),
		}),
	),
});
export const normalizedSchema = v.pipe(
	workspaceSchema,
	v.check(
		(workspace) =>
			workspace.projects.every((project) =>
				project.phases.every((phase) => phase.tasks.length > 0),
			),
		"Every phase needs tasks",
	),
	v.transform((workspace) => ({
		...workspace,
		projects: workspace.projects.map((project) => ({
			...project,
			tasks: project.phases.flatMap((phase) =>
				phase.tasks.map((task) => ({ ...task, phaseId: phase.id })),
			),
			budget: project.phases.reduce(
				(sum, phase) => sum + phase.budget.amount,
				0,
			),
		})),
		emails: workspace.organization.contacts.map((contact) => contact.email),
	})),
);
export const draftSchema = v.partial(workspaceSchema);
export type WorkspaceInput = v.InferInput<typeof workspaceSchema>;
export type Workspace = v.InferOutput<typeof normalizedSchema>;
export type Project = Workspace["projects"][number];
export type Task = Project["tasks"][number];
export type TaskByKind = {
	[Kind in Task["kind"]]: Extract<Task, { kind: Kind }>;
};
export type Draft = v.InferInput<typeof draftSchema>;
