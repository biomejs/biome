import {
	hc,
	type ClientResponse,
	type InferRequestType,
	type InferResponseType,
} from "./vendor/hono";
import type { AppType } from "./server";
import type {
	CreateProjectInput,
	CreateTaskInput,
	Priority,
	TaskStatus,
} from "./schema";

export interface Session {
	token: string;
	orgId: string;
	baseUrl: `https://${string}`;
}

export const createClient = (session: Session) =>
	hc<AppType>(session.baseUrl, {
		headers: () => ({
			authorization: `Bearer ${session.token}`,
			"x-tenant-id": session.orgId,
		}),
	});

export type Client = ReturnType<typeof createClient>;
type Api = Client["api"]["v1"];
type OrgApi = Api["orgs"][":orgId"];
type ProjectsApi = OrgApi["projects"];
type ProjectApi = ProjectsApi[":projectId"];
type TasksApi = ProjectApi["tasks"];
type TaskApi = TasksApi[":taskId"];
type CommentsApi = TaskApi["comments"];

export type Me = InferResponseType<Api["me"]["$get"], 200>;
export type Org = InferResponseType<OrgApi["$get"], 200>;
export type ProjectLookup = InferResponseType<
	ProjectsApi[":projectId?"]["$get"],
	200
>;
export type ProjectPage = Extract<ProjectLookup, { kind: "page" }>;
export type ProjectDto = Extract<ProjectLookup, { kind: "project" }>["project"];
export type ProjectCreateResult = InferResponseType<ProjectsApi["$post"]>;
export type ProjectCreateRequest = InferRequestType<ProjectsApi["$post"]>;
export type ProjectPatchRequest = InferRequestType<ProjectApi["$patch"]>;
export type TaskPage = InferResponseType<TasksApi["$get"], 200>;
export type TaskDto = InferResponseType<TaskApi["$get"], 200>;
export type TaskCreateRequest = InferRequestType<TasksApi["$post"]>;
export type TaskCreated = InferResponseType<TasksApi["$post"], 201>;
export type TaskConflict = InferResponseType<TaskApi["$patch"], 409>;
export type BulkResult = InferResponseType<
	TasksApi["bulk"]["$post"],
	200 | 207
>;
export type ValidationFailure = InferResponseType<TasksApi["$post"], 422>;
export type CommentList = InferResponseType<CommentsApi["$get"], 200>;
export type MilestoneBoard = InferResponseType<
	ProjectApi["milestones"]["$get"],
	200
>;
export type Report = InferResponseType<ProjectApi["report"]["$get"], 200>;
export type ReportJson = Exclude<Report, string>;
export type Webhooks = InferResponseType<OrgApi["webhooks"]["$get"], 200>;
export type MemberInvite = InferRequestType<OrgApi["members"]["$post"]>["json"];

export type Outcome<T, E extends string = string> =
	| { ok: true; value: T }
	| { ok: false; code: E; status: number; message: string };

export class ApiRequestError extends Error {
	constructor(
		readonly status: number,
		readonly code: string,
		readonly requestId: string,
	) {
		super(`${status} ${code} (${requestId})`);
	}
}

type ErrorPayload = {
	error: { code: string; message: string; requestId: string };
};

function isErrorPayload(value: unknown): value is ErrorPayload {
	return (
		typeof value === "object" &&
		value !== null &&
		"error" in value &&
		typeof value.error === "object"
	);
}

type SuccessStatus = 200 | 201 | 202 | 204 | 207;
type SuccessBody<R> =
	R extends ClientResponse<infer T, infer U, "json">
		? U extends SuccessStatus
			? T
			: never
		: never;

// Resolves the success body of any endpoint, turning error payloads into
// exceptions. The body type is picked out of the response union by status.
export async function unwrap<R extends ClientResponse<unknown, number, "json">>(
	pending: Promise<R>,
): Promise<SuccessBody<R>> {
	const response = await pending;
	const body: unknown = await response.json();
	if (!response.ok || isErrorPayload(body)) {
		const error = isErrorPayload(body)
			? body.error
			: { code: "unknown", requestId: "n/a" };
		throw new ApiRequestError(response.status, error.code, error.requestId);
	}
	return body as SuccessBody<R>;
}

export async function health(client: Client) {
	const res = await client.api.v1.health.$get();
	const body = await res.json();
	return { healthy: body.status === "ok", uptime: body.uptime };
}

export async function whoAmI(client: Client): Promise<Me> {
	const res = await client.api.v1.me.$get();
	if (res.status !== 200) throw new ApiRequestError(res.status, "me", "n/a");
	return res.json();
}

export async function loadOrg(client: Client, orgId: string) {
	const res = await client.api.v1.orgs[":orgId"].$get({ param: { orgId } });
	if (res.status === 200) {
		const org = await res.json();
		return {
			...org,
			canUseReports: org.features.includes("reports"),
			isPaid: org.plan !== "free",
		};
	}
	const failure = await res.json();
	throw new ApiRequestError(
		res.status,
		failure.error.code,
		failure.error.requestId,
	);
}

export async function listProjects(
	client: Client,
	orgId: string,
	filter: {
		search?: string;
		visibility?: ("private" | "internal" | "public")[];
		archived?: boolean;
		sort?: `${"-" | ""}${"name" | "created_at" | "updated_at" | "task_count"}`;
		page?: number;
	} = {},
): Promise<ProjectPage> {
	const res = await client.api.v1.orgs[":orgId"].projects[":projectId?"].$get({
		param: { orgId },
		query: {
			search: filter.search,
			visibility: filter.visibility?.join(","),
			archived:
				filter.archived === undefined ? undefined : `${filter.archived}`,
			sort: filter.sort,
			page: filter.page?.toString(),
			pageSize: "50",
		},
	});
	switch (res.status) {
		case 200: {
			const body = await res.json();
			if (body.kind !== "page") throw new Error("Expected a page");
			return body;
		}
		case 404:
		case 422: {
			const body = await res.json();
			throw new ApiRequestError(
				res.status,
				body.error.code,
				body.error.requestId,
			);
		}
	}
}

export async function* allProjects(client: Client, orgId: string) {
	for (let page = 1; ; page++) {
		const batch = await listProjects(client, orgId, { page });
		yield* batch.items;
		if (batch.nextCursor === null) return;
	}
}

export async function getProject(
	client: Client,
	orgId: string,
	projectId: string,
): Promise<Outcome<ProjectDto, "project_not_found" | "validation_failed">> {
	const res = await client.api.v1.orgs[":orgId"].projects[":projectId?"].$get({
		param: { orgId, projectId },
		query: {},
	});
	if (res.ok) {
		const body = await res.json();
		return body.kind === "project"
			? { ok: true, value: body.project }
			: { ok: false, code: "project_not_found", status: 404, message: "" };
	}
	const { error } = await res.json();
	return {
		ok: false,
		code: error.code,
		status: res.status,
		message: error.message,
	};
}

export async function createProject(
	client: Client,
	orgId: string,
	input: CreateProjectInput,
) {
	const res = await client.api.v1.orgs[":orgId"].projects.$post({
		param: { orgId },
		header: { "idempotency-key": crypto.randomUUID() },
		json: input,
	});
	switch (res.status) {
		case 201: {
			const project = await res.json();
			return { created: true as const, project };
		}
		case 402: {
			const limit = await res.json();
			return {
				created: false as const,
				reason: `Upgrade from ${limit.plan} (limit ${limit.limit})`,
			};
		}
		case 409: {
			const conflict = await res.json();
			return { created: false as const, reason: conflict.error.message };
		}
		case 422: {
			const invalid = await res.json();
			return {
				created: false as const,
				reason: Object.entries(invalid.error.details ?? {})
					.map(([field, messages]) => `${field}: ${messages?.join(", ")}`)
					.join("; "),
			};
		}
	}
}

export async function renameProject(
	client: Client,
	session: Session,
	projectId: string,
	name: string,
	etag?: string,
) {
	const request: ProjectPatchRequest = {
		param: { orgId: session.orgId, projectId },
		header: {
			authorization: `Bearer ${session.token}`,
			"x-tenant-id": session.orgId,
			"if-match": etag,
		},
		json: { name },
	};
	const res =
		await client.api.v1.orgs[":orgId"].projects[":projectId"].$patch(request);
	if (res.status === 412) {
		const stale = await res.json();
		return renameProject(client, session, projectId, name, stale.etag);
	}
	return unwrap(Promise.resolve(res));
}

export async function archiveProject(
	client: Client,
	orgId: string,
	projectId: string,
	mergeInto?: string,
) {
	const res = await client.api.v1.orgs[":orgId"].projects[
		":projectId"
	].archive.$post({
		param: { orgId, projectId },
		json: mergeInto ? { reason: "merged", mergeInto } : { reason: "completed" },
	});
	if (res.status === 200) {
		const { archived, reason } = await res.json();
		return `${archived.name} archived (${reason}) at ${archived.archivedAt}`;
	}
	if (res.status === 400 || res.status === 404) {
		return (await res.json()).error.code;
	}
	return null;
}

export const projectLinks = (
	client: Client,
	orgId: string,
	projectId: string,
) => {
	const project = client.api.v1.orgs[":orgId"].projects[":projectId"];
	return {
		self: project.$url({ param: { orgId, projectId } }),
		tasks: project.tasks.$url({ param: { orgId, projectId } }),
		report: project.report.$url({
			param: { orgId, projectId },
			query: { from: "2026-01-01", to: "2026-03-31", format: "csv" },
		}),
		activity: project.activity.$path({
			param: { orgId, projectId },
			query: { limit: "20" },
		}),
	};
};

export async function listTasks(
	client: Client,
	orgId: string,
	projectId: string,
	filter: {
		status?: TaskStatus[];
		priority?: Priority[];
		assignee?: "me" | "none" | (string & {});
		labels?: string[];
		dueBefore?: string;
		sort?: "priority" | "-priority" | "due_date" | "rank";
	},
): Promise<TaskPage> {
	return unwrap(
		client.api.v1.orgs[":orgId"].projects[":projectId"].tasks.$get({
			param: { orgId, projectId },
			query: {
				status: filter.status?.join(","),
				priority: filter.priority?.join(","),
				assignee: filter.assignee,
				label: filter.labels,
				dueBefore: filter.dueBefore,
				sort: filter.sort,
				includeSubtasks: "false",
			},
		}),
	);
}

export async function fileBug(
	client: Client,
	orgId: string,
	projectId: string,
	title: string,
	steps: string[],
) {
	const json = {
		kind: "bug",
		title,
		severity: steps.length > 0 ? "s2" : "s1",
		priority: "high",
		environment: "production",
		reproduction:
			steps.length > 0
				? { steps, expected: "It works", actual: "It does not" }
				: undefined,
	} satisfies CreateTaskInput;
	const res = await client.api.v1.orgs[":orgId"].projects[
		":projectId"
	].tasks.$post({
		param: { orgId, projectId },
		header: { "idempotency-key": crypto.randomUUID() },
		json,
	});
	if (res.status === 201) {
		const task = await res.json();
		if (task.kind === "bug") {
			return { id: task.id, number: task.number, severity: task.severity };
		}
		throw new Error(`Created a ${task.kind} instead of a bug`);
	}
	if (res.status === 422) {
		const failure: ValidationFailure = await res.json();
		throw new ApiRequestError(422, failure.error.code, failure.error.requestId);
	}
	const other = await res.json();
	throw new ApiRequestError(
		res.status,
		other.error.code,
		other.error.requestId,
	);
}

export async function planFeature(
	client: Client,
	orgId: string,
	projectId: string,
	title: string,
	criteria: string[],
	epicId?: string,
): Promise<TaskCreated> {
	const request: TaskCreateRequest = {
		param: { orgId, projectId },
		header: { "idempotency-key": crypto.randomUUID() },
		json: {
			kind: "feature",
			title,
			acceptanceCriteria: criteria,
			epicId,
			status: "backlog",
			estimate: criteria.length * 2,
		},
	};
	return unwrap(
		client.api.v1.orgs[":orgId"].projects[":projectId"].tasks.$post(request),
	);
}

export async function scheduleChore(
	client: Client,
	orgId: string,
	projectId: string,
	title: string,
	every: number,
	unit: "day" | "week" | "month",
) {
	const created = await unwrap(
		client.api.v1.orgs[":orgId"].projects[":projectId"].tasks.$post({
			param: { orgId, projectId },
			header: { "idempotency-key": crypto.randomUUID() },
			json: { kind: "chore", title, recurring: { every, unit } },
		}),
	);
	return created.kind === "chore" ? created.recurring : undefined;
}

export async function getTask(
	client: Client,
	orgId: string,
	projectId: string,
	taskId: string,
): Promise<TaskDto | null> {
	const res = await client.api.v1.orgs[":orgId"].projects[":projectId"].tasks[
		":taskId"
	].$get({ param: { orgId, projectId, taskId } });
	return res.status === 200 ? res.json() : null;
}

export async function updateTaskWithRetry(
	client: Client,
	orgId: string,
	projectId: string,
	taskId: string,
	change: (task: TaskDto | TaskConflict["current"]) => {
		status?: TaskStatus;
		priority?: Priority;
		assigneeIds?: string[];
	},
	attempts = 3,
) {
	const endpoint =
		client.api.v1.orgs[":orgId"].projects[":projectId"].tasks[":taskId"];
	let current: TaskDto | TaskConflict["current"] | null = await getTask(
		client,
		orgId,
		projectId,
		taskId,
	);
	for (let attempt = 0; current && attempt < attempts; attempt++) {
		const res = await endpoint.$patch({
			param: { orgId, projectId, taskId },
			json: { ...change(current), version: current.version },
		});
		if (res.status === 200) return res.json();
		if (res.status !== 409) break;
		const conflict = await res.json();
		current = conflict.current;
	}
	return null;
}

export async function moveTask(
	client: Client,
	orgId: string,
	projectId: string,
	taskId: string,
	status: TaskStatus,
	neighbours: { before?: string; after?: string } = {},
) {
	const res = await client.api.v1.orgs[":orgId"].projects[":projectId"].tasks[
		":taskId"
	].position.$put({
		param: { orgId, projectId, taskId },
		json: {
			status,
			beforeId: neighbours.before ?? null,
			afterId: neighbours.after ?? null,
		},
	});
	if (!res.ok) return undefined;
	const moved = await res.json();
	return `${moved.id} -> ${moved.status} @ ${moved.rank}`;
}

export async function closeAll(
	client: Client,
	orgId: string,
	projectId: string,
	taskIds: string[],
) {
	const res = await client.api.v1.orgs[":orgId"].projects[
		":projectId"
	].tasks.bulk.$post({
		param: { orgId, projectId },
		json: {
			taskIds,
			operation: { op: "close", resolution: "fixed" },
		},
	});
	if (res.status === 200 || res.status === 207) {
		const body: BulkResult = await res.json();
		const failures = body.results.filter((result) => !result.ok);
		return {
			closed: body.results.length - failures.length,
			failed: failures.map((failure) => failure.id),
		};
	}
	return { closed: 0, failed: taskIds };
}

export async function relabel(
	client: Client,
	orgId: string,
	projectId: string,
	taskIds: string[],
	add: string[],
	remove: string[],
) {
	return unwrap(
		client.api.v1.orgs[":orgId"].projects[":projectId"].tasks.bulk.$post({
			param: { orgId, projectId },
			json: { taskIds, operation: { op: "label", add, remove } },
		}),
	);
}

export async function discuss(
	client: Client,
	ids: { orgId: string; projectId: string; taskId: string },
	body: string,
	mentions: string[] = [],
) {
	const comments =
		client.api.v1.orgs[":orgId"].projects[":projectId"].tasks[":taskId"]
			.comments;
	const posted = await comments.$post({
		param: ids,
		json: { body, mentions },
	});
	if (posted.status !== 201) return null;
	const { comment, deliveries } = await posted.json();
	const thread: CommentList = await unwrap(comments.$get({ param: ids }));
	const edited = await comments[":commentId"].$patch({
		param: { ...ids, commentId: comment.id },
		json: { body: `${body} (edited)` },
	});
	const removed = await comments[":commentId"].$delete({
		param: { ...ids, commentId: comment.id },
	});
	return {
		commentId: comment.id,
		deliveries,
		threadSize: thread.total,
		edited: edited.status === 200 ? (await edited.json()).editedAt : null,
		removed: removed.status === 204,
	};
}

export async function projectDashboard(
	client: Client,
	orgId: string,
	projectId: string,
) {
	const project = client.api.v1.orgs[":orgId"].projects[":projectId"];
	const param = { orgId, projectId };
	const [labels, milestones, activity, report] = await Promise.all([
		unwrap(project.labels.$get({ param })),
		project.milestones.$get({ param }),
		project.activity.$get({ param, query: { limit: "50" } }),
		project.report.$get({
			param,
			query: { from: "2026-07-01", to: "2026-09-30", groupBy: "status" },
		}),
	]);
	const board: MilestoneBoard | null =
		milestones.status === 200 ? await milestones.json() : null;
	const events = activity.ok ? (await activity.json()).events : [];
	let rows: ReportJson["rows"] = [];
	if (report.status === 200) {
		rows = (await report.json()).rows;
	} else if (report.status === 402) {
		const upgrade = await report.json();
		console.warn(`Reports require an upgrade from ${upgrade.plan}`);
	}
	return {
		labels: labels.map((label) => `${label.name}#${label.color}`),
		nextMilestone: board?.open[0]?.title,
		recentlyClosed: events.filter((event) => event.type === "task.closed"),
		statusCounts: Object.fromEntries(
			rows.map((row) => [row.status, row.count]),
		),
	};
}

export async function createMilestone(
	client: Client,
	orgId: string,
	projectId: string,
	title: string,
	dueOn: string,
) {
	const milestones =
		client.api.v1.orgs[":orgId"].projects[":projectId"].milestones;
	const created = await unwrap(
		milestones.$post({ param: { orgId, projectId }, json: { title, dueOn } }),
	);
	const closed = await milestones[":milestoneId"].$patch({
		param: { orgId, projectId, milestoneId: created.id },
		json: { state: "closed" },
	});
	return closed.status === 200 ? (await closed.json()).state : created.state;
}

export async function inviteTeam(
	client: Client,
	orgId: string,
	invites: MemberInvite[],
) {
	const members = client.api.v1.orgs[":orgId"].members;
	const results = await Promise.all(
		invites.map(async (json) => {
			const res = await members.$post({ param: { orgId }, json });
			switch (res.status) {
				case 201:
					return { email: json.email, url: (await res.json()).inviteUrl };
				case 402:
				case 422:
					return { email: json.email, error: (await res.json()).error.code };
			}
		}),
	);
	const roster = await unwrap(members.$get({ param: { orgId } }));
	return { results, owners: roster.owners, size: roster.members.length };
}

export async function promote(
	client: Client,
	orgId: string,
	userId: string,
	role: "owner" | "admin" | "member" | "guest",
) {
	const res = await client.api.v1.orgs[":orgId"].members[":userId"].$patch({
		param: { orgId, userId },
		json: { role },
	});
	if (res.status === 409) return "last owner" as const;
	if (res.status === 200) return (await res.json()).role;
	return undefined;
}

export async function registerWebhook(
	client: Client,
	orgId: string,
	url: `https://${string}`,
) {
	const webhooks = client.api.v1.orgs[":orgId"].webhooks;
	const res = await webhooks.$post({
		param: { orgId },
		json: {
			url,
			events: ["task.created", "task.closed", "comment.created"],
			filters: { priorities: ["high", "urgent"] },
		},
	});
	if (res.status === 402) {
		const { feature, plan } = await res.json();
		return { registered: false as const, feature, plan };
	}
	if (res.status !== 201) return { registered: false as const };
	const hook = await res.json();
	const ping = await webhooks[":webhookId"].test.$post({
		param: { orgId, webhookId: hook.id },
	});
	const delivery = await ping.json();
	const existing: Webhooks = await unwrap(webhooks.$get({ param: { orgId } }));
	return {
		registered: true as const,
		id: hook.id,
		delivered: "delivered" in delivery ? delivery.delivered : false,
		total: existing.length,
	};
}
