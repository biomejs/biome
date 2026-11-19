import * as v from "./vendor/valibot";

const prefixedId = <const Prefix extends string>(prefix: Prefix) =>
	v.pipe(
		v.string(),
		v.trim(),
		v.regex(new RegExp(`^${prefix}_[a-z0-9]{12}$`)),
		v.brand(`${prefix}Id`),
	);

export const OrgId = prefixedId("org");
export const ProjectId = prefixedId("prj");
export const TaskId = prefixedId("tsk");
export const UserId = prefixedId("usr");
export const CommentId = prefixedId("cmt");
export const LabelId = prefixedId("lbl");
export const MilestoneId = prefixedId("mst");
export const WebhookId = prefixedId("whk");

export type OrgId = v.InferOutput<typeof OrgId>;
export type ProjectId = v.InferOutput<typeof ProjectId>;
export type TaskId = v.InferOutput<typeof TaskId>;
export type UserId = v.InferOutput<typeof UserId>;
export type CommentId = v.InferOutput<typeof CommentId>;
export type LabelId = v.InferOutput<typeof LabelId>;
export type MilestoneId = v.InferOutput<typeof MilestoneId>;
export type WebhookId = v.InferOutput<typeof WebhookId>;

export const Role = v.picklist(["owner", "admin", "member", "guest"]);
export const TaskStatus = v.picklist([
	"backlog",
	"todo",
	"in_progress",
	"in_review",
	"blocked",
	"done",
	"cancelled",
]);
export const Priority = v.picklist(["none", "low", "medium", "high", "urgent"]);
export const Visibility = v.picklist(["private", "internal", "public"]);
export const WebhookEvent = v.picklist([
	"project.created",
	"project.updated",
	"project.archived",
	"task.created",
	"task.updated",
	"task.moved",
	"task.closed",
	"comment.created",
	"member.invited",
]);

export type Role = v.InferOutput<typeof Role>;
export type TaskStatus = v.InferOutput<typeof TaskStatus>;
export type Priority = v.InferOutput<typeof Priority>;
export type Visibility = v.InferOutput<typeof Visibility>;
export type WebhookEvent = v.InferOutput<typeof WebhookEvent>;

const Title = v.pipe(v.string(), v.trim(), v.minLength(1), v.maxLength(200));
const Markdown = v.pipe(v.string(), v.maxLength(50_000));
const Slug = v.pipe(
	v.string(),
	v.trim(),
	v.toLowerCase(),
	v.regex(/^[a-z0-9]+(?:-[a-z0-9]+)*$/),
	v.maxLength(64),
);
const HexColor = v.pipe(v.string(), v.hexColor());
const Timestamp = v.pipe(v.string(), v.isoTimestamp());
const CalendarDate = v.pipe(v.string(), v.isoDate());
const Estimate = v.pipe(
	v.number(),
	v.integer(),
	v.minValue(0),
	v.maxValue(100),
);

// Every query and path value arrives as a string, so numeric and boolean
// inputs are parsed by transformations whose input type stays `string`.
const IntFromQuery = (min: number, max: number) =>
	v.pipe(
		v.string(),
		v.digits(),
		v.transform(Number),
		v.integer(),
		v.minValue(min),
		v.maxValue(max),
	);
const BoolFromQuery = v.pipe(
	v.picklist(["true", "false", "1", "0"]),
	v.transform((value) => value === "true" || value === "1"),
);
const CsvList = <const TOptions extends readonly [string, ...string[]]>(
	options: TOptions,
) =>
	v.pipe(
		v.string(),
		v.transform((value) => value.split(",").filter(Boolean)),
		v.array(v.picklist(options)),
		v.minLength(1),
	);
const Sort = <const TFields extends readonly [string, ...string[]]>(
	fields: TFields,
) =>
	v.pipe(
		v.string(),
		v.regex(/^-?[a-z_]+$/),
		v.transform((value) => {
			const descending = value.startsWith("-");
			return {
				field: (descending ? value.slice(1) : value) as TFields[number],
				direction: descending ? ("desc" as const) : ("asc" as const),
			};
		}),
		v.check(
			(sort) => (fields as readonly string[]).includes(sort.field),
			"Unknown sort field",
		),
	);

export const OrgParam = v.object({ orgId: OrgId });
export const OptionalProjectParam = v.object({
	orgId: OrgId,
	projectId: v.optional(ProjectId),
});
export const ProjectParam = v.object({ orgId: OrgId, projectId: ProjectId });
export const TaskParam = v.object({
	orgId: OrgId,
	projectId: ProjectId,
	taskId: TaskId,
});
export const CommentParam = v.object({
	orgId: OrgId,
	projectId: ProjectId,
	taskId: TaskId,
	commentId: CommentId,
});
export const MemberParam = v.object({ orgId: OrgId, userId: UserId });
export const LabelParam = v.object({
	orgId: OrgId,
	projectId: ProjectId,
	labelId: LabelId,
});
export const MilestoneParam = v.object({
	orgId: OrgId,
	projectId: ProjectId,
	milestoneId: MilestoneId,
});
export const WebhookParam = v.object({ orgId: OrgId, webhookId: WebhookId });

export const AuthHeaders = v.object({
	authorization: v.pipe(v.string(), v.startsWith("Bearer "), v.minLength(20)),
	"x-tenant-id": OrgId,
	"x-request-id": v.optional(v.pipe(v.string(), v.uuid())),
	"if-match": v.optional(v.pipe(v.string(), v.regex(/^W\/"\d+"$/))),
});
export const IdempotencyHeaders = v.object({
	"idempotency-key": v.pipe(v.string(), v.uuid()),
});

export const Pagination = v.object({
	page: v.optional(IntFromQuery(1, 10_000), "1"),
	pageSize: v.optional(IntFromQuery(1, 200), "25"),
	cursor: v.optional(v.pipe(v.string(), v.base64())),
});

export const ProjectListQuery = v.object({
	...Pagination.entries,
	search: v.optional(v.pipe(v.string(), v.trim(), v.minLength(2))),
	visibility: v.optional(CsvList(Visibility.options)),
	archived: v.optional(BoolFromQuery),
	sort: v.optional(Sort(["name", "created_at", "updated_at", "task_count"])),
	owner: v.optional(UserId),
});

export const TaskListQuery = v.object({
	...Pagination.entries,
	status: v.optional(CsvList(TaskStatus.options)),
	priority: v.optional(CsvList(Priority.options)),
	assignee: v.optional(v.union([v.literal("me"), v.literal("none"), UserId])),
	label: v.optional(v.union([LabelId, v.array(LabelId)])),
	milestone: v.optional(MilestoneId),
	dueBefore: v.optional(CalendarDate),
	dueAfter: v.optional(CalendarDate),
	includeSubtasks: v.optional(BoolFromQuery),
	sort: v.optional(
		Sort(["priority", "status", "due_date", "created_at", "rank"]),
	),
});

export const ActivityQuery = v.object({
	since: v.optional(Timestamp),
	until: v.optional(Timestamp),
	actor: v.optional(UserId),
	types: v.optional(CsvList(WebhookEvent.options)),
	limit: v.optional(IntFromQuery(1, 500), "100"),
});

export const ReportQuery = v.object({
	from: CalendarDate,
	to: CalendarDate,
	groupBy: v.optional(v.picklist(["assignee", "status", "priority", "label"])),
	format: v.optional(v.picklist(["json", "csv"]), "json"),
});

export const ProjectSettings = v.object({
	defaultStatus: v.optional(TaskStatus, "todo"),
	defaultPriority: v.optional(Priority, "none"),
	allowGuests: v.optional(v.boolean(), false),
	workflow: v.optional(
		v.pipe(
			v.array(
				v.object({
					from: TaskStatus,
					to: v.pipe(v.array(TaskStatus), v.minLength(1)),
				}),
			),
			v.maxLength(32),
		),
	),
	estimation: v.optional(
		v.variant("scale", [
			v.object({ scale: v.literal("points"), max: Estimate }),
			v.object({
				scale: v.literal("tshirt"),
				sizes: v.array(v.picklist(["xs", "s", "m", "l", "xl"])),
			}),
			v.object({ scale: v.literal("hours"), maxPerTask: v.number() }),
		]),
	),
});

export const CreateProject = v.object({
	name: Title,
	slug: Slug,
	description: v.optional(Markdown),
	visibility: v.optional(Visibility, "private"),
	color: v.optional(HexColor),
	leadId: v.optional(UserId),
	memberIds: v.optional(v.pipe(v.array(UserId), v.maxLength(500)), []),
	settings: v.optional(ProjectSettings),
	startsOn: v.optional(CalendarDate),
	endsOn: v.optional(CalendarDate),
});

export const UpdateProject = v.pipe(
	v.partial(v.omit(CreateProject, ["slug", "memberIds"])),
	v.check(
		(input) => Object.keys(input).length > 0,
		"At least one field must be provided",
	),
);

export const ArchiveProject = v.object({
	reason: v.optional(v.picklist(["completed", "abandoned", "merged"])),
	mergeInto: v.optional(ProjectId),
});

const TaskBase = v.object({
	title: Title,
	description: v.optional(Markdown),
	status: v.optional(TaskStatus),
	priority: v.optional(Priority),
	assigneeIds: v.optional(v.pipe(v.array(UserId), v.maxLength(10))),
	labelIds: v.optional(v.array(LabelId)),
	milestoneId: v.optional(v.nullable(MilestoneId)),
	parentId: v.optional(v.nullable(TaskId)),
	dueOn: v.optional(v.nullable(CalendarDate)),
	estimate: v.optional(Estimate),
});

export const CreateTask = v.variant("kind", [
	v.object({
		...TaskBase.entries,
		kind: v.literal("bug"),
		severity: v.picklist(["s1", "s2", "s3", "s4"]),
		environment: v.optional(v.picklist(["production", "staging", "local"])),
		reproduction: v.optional(
			v.object({
				steps: v.pipe(v.array(v.string()), v.minLength(1)),
				expected: v.string(),
				actual: v.string(),
			}),
		),
	}),
	v.object({
		...TaskBase.entries,
		kind: v.literal("feature"),
		acceptanceCriteria: v.optional(v.array(v.string()), []),
		epicId: v.optional(TaskId),
	}),
	v.object({
		...TaskBase.entries,
		kind: v.literal("chore"),
		recurring: v.optional(
			v.object({
				every: v.pipe(v.number(), v.integer(), v.minValue(1)),
				unit: v.picklist(["day", "week", "month"]),
			}),
		),
	}),
]);

export const UpdateTask = v.object({
	...v.partial(TaskBase).entries,
	version: v.pipe(v.number(), v.integer(), v.minValue(0)),
});

export const MoveTask = v.object({
	status: TaskStatus,
	beforeId: v.optional(v.nullable(TaskId)),
	afterId: v.optional(v.nullable(TaskId)),
	projectId: v.optional(ProjectId),
});

export const BulkTaskOperation = v.object({
	taskIds: v.pipe(v.array(TaskId), v.minLength(1), v.maxLength(100)),
	operation: v.variant("op", [
		v.object({ op: v.literal("move"), status: TaskStatus }),
		v.object({ op: v.literal("assign"), assigneeIds: v.array(UserId) }),
		v.object({
			op: v.literal("label"),
			add: v.optional(v.array(LabelId), []),
			remove: v.optional(v.array(LabelId), []),
		}),
		v.object({ op: v.literal("prioritize"), priority: Priority }),
		v.object({
			op: v.literal("close"),
			resolution: v.picklist(["fixed", "wontfix", "duplicate"]),
		}),
	]),
});

export const CreateComment = v.object({
	body: v.pipe(Markdown, v.minLength(1)),
	mentions: v.optional(v.array(UserId), []),
	replyTo: v.optional(CommentId),
});

export const UpdateComment = v.pick(CreateComment, ["body", "mentions"]);

export const InviteMember = v.object({
	email: v.pipe(v.string(), v.trim(), v.toLowerCase(), v.email()),
	role: v.optional(v.picklist(["admin", "member", "guest"]), "member"),
	projectIds: v.optional(v.array(ProjectId), []),
	message: v.optional(v.pipe(v.string(), v.maxLength(500))),
});

export const UpdateMember = v.object({
	role: Role,
	projectIds: v.optional(v.array(ProjectId)),
});

export const CreateLabel = v.object({
	name: v.pipe(Title, v.maxLength(40)),
	color: HexColor,
	description: v.optional(v.string()),
});

export const CreateMilestone = v.object({
	title: Title,
	dueOn: CalendarDate,
	description: v.optional(Markdown),
	state: v.optional(v.picklist(["open", "closed"]), "open"),
});

export const CreateWebhook = v.object({
	url: v.pipe(v.string(), v.url(), v.startsWith("https://")),
	events: v.pipe(v.array(WebhookEvent), v.minLength(1)),
	secret: v.optional(v.pipe(v.string(), v.minLength(16))),
	active: v.optional(v.boolean(), true),
	filters: v.optional(
		v.object({
			projectIds: v.optional(v.array(ProjectId)),
			priorities: v.optional(v.array(Priority)),
		}),
	),
});

export type ProjectListQuery = v.InferOutput<typeof ProjectListQuery>;
export type TaskListQuery = v.InferOutput<typeof TaskListQuery>;
export type CreateProjectInput = v.InferInput<typeof CreateProject>;
export type CreateProject = v.InferOutput<typeof CreateProject>;
export type UpdateProject = v.InferOutput<typeof UpdateProject>;
export type ProjectSettings = v.InferOutput<typeof ProjectSettings>;
export type CreateTaskInput = v.InferInput<typeof CreateTask>;
export type CreateTask = v.InferOutput<typeof CreateTask>;
export type UpdateTask = v.InferOutput<typeof UpdateTask>;
export type MoveTask = v.InferOutput<typeof MoveTask>;
export type BulkTaskOperation = v.InferOutput<typeof BulkTaskOperation>;
export type CreateComment = v.InferOutput<typeof CreateComment>;
export type InviteMember = v.InferOutput<typeof InviteMember>;
export type CreateWebhook = v.InferOutput<typeof CreateWebhook>;

export interface AuthUser {
	id: UserId;
	email: string;
	name: string;
	roles: Record<OrgId, Role>;
	scopes: ReadonlyArray<"read" | "write" | "admin">;
}

export interface Tenant {
	id: OrgId;
	slug: string;
	plan: "free" | "team" | "enterprise";
	limits: { projects: number; membersPerProject: number; webhooks: number };
	features: ReadonlySet<"reports" | "webhooks" | "guests" | "sso">;
}

export interface Project {
	id: ProjectId;
	orgId: OrgId;
	name: string;
	slug: string;
	description: string | null;
	visibility: Visibility;
	color: string | null;
	leadId: UserId | null;
	memberIds: UserId[];
	settings: ProjectSettings;
	archivedAt: string | null;
	createdAt: string;
	updatedAt: string;
	taskCount: number;
}

type DistributiveOmit<T, K extends PropertyKey> = T extends unknown
	? Omit<T, K>
	: never;

export type Task = DistributiveOmit<CreateTask, "status" | "priority"> & {
	id: TaskId;
	projectId: ProjectId;
	number: number;
	status: TaskStatus;
	priority: Priority;
	reporterId: UserId;
	version: number;
	rank: string;
	closedAt: string | null;
	createdAt: string;
	updatedAt: string;
};

export interface Comment {
	id: CommentId;
	taskId: TaskId;
	authorId: UserId;
	body: string;
	mentions: UserId[];
	replyTo: CommentId | null;
	editedAt: string | null;
	createdAt: string;
}

export interface Member {
	userId: UserId;
	orgId: OrgId;
	email: string;
	role: Role;
	projectIds: ProjectId[];
	invitedAt: string;
	joinedAt: string | null;
}

export interface Label {
	id: LabelId;
	projectId: ProjectId;
	name: string;
	color: string;
	description: string | null;
}

export interface Milestone {
	id: MilestoneId;
	projectId: ProjectId;
	title: string;
	dueOn: string;
	state: "open" | "closed";
	progress: { total: number; done: number };
}

export interface Webhook {
	id: WebhookId;
	orgId: OrgId;
	url: string;
	events: WebhookEvent[];
	active: boolean;
	secretLast4: string | null;
	createdAt: string;
}

export interface Page<T> {
	items: T[];
	page: number;
	pageSize: number;
	total: number;
	nextCursor: string | null;
}

export interface ApiError<Code extends string = string> {
	error: {
		code: Code;
		message: string;
		requestId: string;
		details?: Record<string, string[] | undefined>;
	};
}
