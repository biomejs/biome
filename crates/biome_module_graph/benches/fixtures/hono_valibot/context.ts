import {
	HTTPException,
	createMiddleware,
	vValidator,
	type Context,
} from "./vendor/hono";
import * as v from "./vendor/valibot";
import {
	TaskListQuery,
	type ApiError,
	type AuthUser,
	type Comment,
	type Label,
	type Member,
	type Milestone,
	type OrgId,
	type Page,
	type Project,
	type ProjectId,
	type Role,
	type Task,
	type TaskId,
	type Tenant,
	type UserId,
	type Webhook,
	type WebhookEvent,
} from "./schema";

type Bindings = {
	DATABASE_URL: string;
	SIGNING_KEY: string;
	PLAN_OVERRIDES?: string;
};

type RequestVariables = { requestId: string; startedAt: number };
type AuthVariables = { user: AuthUser };
type TenantVariables = { tenant: Tenant; role: Role };

export type AppEnv = {
	Bindings: Bindings;
	Variables: RequestVariables & AuthVariables & TenantVariables;
};

const roleRank = {
	guest: 0,
	member: 1,
	admin: 2,
	owner: 3,
} as const satisfies Record<Role, number>;

export const db = {
	users: new Map<string, AuthUser>(),
	tenants: new Map<OrgId, Tenant>(),
	projects: new Map<ProjectId, Project>(),
	tasks: new Map<TaskId, Task>(),
	comments: new Map<string, Comment>(),
	members: new Map<`${OrgId}:${UserId}`, Member>(),
	labels: new Map<string, Label>(),
	milestones: new Map<string, Milestone>(),
	webhooks: new Map<string, Webhook>(),
	idempotency: new Map<string, { status: 201; body: Project }>(),
};

export const now = () => new Date().toISOString();
export const newId = <const P extends string>(prefix: P) =>
	`${prefix}_${Math.random().toString(36).slice(2, 14).padEnd(12, "0")}` as const;

export function errorBody<const Code extends string>(
	c: { readonly var: { readonly requestId: string } },
	code: Code,
	message: string,
	details?: Record<string, string[] | undefined>,
): ApiError<Code> {
	return {
		error: { code, message, requestId: c.var.requestId, details },
	};
}

export function paginate<T>(
	items: readonly T[],
	query: { page: number; pageSize: number },
): Page<T> {
	const start = (query.page - 1) * query.pageSize;
	const slice = items.slice(start, start + query.pageSize);
	return {
		items: slice,
		page: query.page,
		pageSize: query.pageSize,
		total: items.length,
		nextCursor:
			start + slice.length < items.length ? btoa(String(query.page + 1)) : null,
	};
}

export function compareBy<T, K extends keyof T>(
	key: K,
	direction: "asc" | "desc",
): (a: T, b: T) => number {
	const sign = direction === "asc" ? 1 : -1;
	return (a, b) => (a[key] < b[key] ? -sign : a[key] > b[key] ? sign : 0);
}

// Every validator failure is reported the same way, so the hook's typed 422
// response is merged into the schema of each route that uses it.
type Issues = [v.BaseIssue<unknown>, ...v.BaseIssue<unknown>[]];
const validationHook = (
	result: { success: boolean; issues?: Issues; target: string },
	c: Context<AppEnv>,
) => {
	if (!result.success && result.issues) {
		const flat = v.flatten(result.issues);
		return c.json(
			errorBody(c, "validation_failed", `Invalid ${result.target}`, {
				root: flat.root,
				...flat.nested,
			}),
			422,
		);
	}
};

export const json = <T extends v.GenericSchema>(schema: T) =>
	vValidator("json", schema, validationHook);
export const query = <T extends v.GenericSchema>(schema: T) =>
	vValidator("query", schema, validationHook);
export const param = <T extends v.GenericSchema>(schema: T) =>
	vValidator("param", schema, validationHook);
export const header = <T extends v.GenericSchema>(schema: T) =>
	vValidator("header", schema, validationHook);

export const requestContext = createMiddleware<{
	Variables: RequestVariables;
}>(async (c, next) => {
	c.set("requestId", c.req.header("x-request-id") ?? crypto.randomUUID());
	c.set("startedAt", performance.now());
	await next();
	c.header("x-request-id", c.var.requestId);
	c.header(
		"server-timing",
		`app;dur=${(performance.now() - c.var.startedAt).toFixed(1)}`,
	);
});

export const authenticate = createMiddleware<{
	Variables: RequestVariables & AuthVariables;
}>(async (c, next) => {
	const token = c.req.header("authorization")?.replace(/^Bearer\s+/i, "");
	if (!token) {
		return c.json(errorBody(c, "unauthenticated", "Missing token"), 401);
	}
	const user = db.users.get(token);
	if (!user) {
		return c.json(errorBody(c, "invalid_token", "Token expired"), 401);
	}
	c.set("user", user);
	await next();
});

export const resolveTenant = createMiddleware<
	{ Variables: RequestVariables & AuthVariables & TenantVariables },
	"/orgs/:orgId/*"
>(async (c, next) => {
	const orgId = c.req.param("orgId") as OrgId;
	const tenant = db.tenants.get(orgId);
	if (!tenant) {
		throw new HTTPException(404, { message: `Unknown org ${orgId}` });
	}
	const role = c.var.user.roles[orgId];
	if (!role) {
		return c.json(errorBody(c, "forbidden", "Not a member of this org"), 403);
	}
	c.set("tenant", tenant);
	c.set("role", role);
	await next();
});

export const requireRole = (minimum: Role) =>
	createMiddleware<AppEnv>(async (c, next) => {
		if (roleRank[c.var.role] < roleRank[minimum]) {
			throw new HTTPException(403, {
				message: `Requires ${minimum}, have ${c.var.role}`,
			});
		}
		await next();
	});

export const requireFeature = <
	const F extends (Tenant["features"] extends ReadonlySet<infer U> ? U : never),
>(
	feature: F,
) =>
	createMiddleware(async (c: Context<AppEnv>, next) => {
		if (!c.var.tenant.features.has(feature)) {
			return c.json(
				{
					...errorBody(c, "plan_upgrade_required", `${feature} is unavailable`),
					plan: c.var.tenant.plan,
					feature,
				},
				402,
			);
		}
		await next();
	});

export function loadProject(orgId: OrgId, projectId: ProjectId) {
	const project = db.projects.get(projectId);
	return project && project.orgId === orgId && project.archivedAt === null
		? project
		: undefined;
}

export function taskMatches(
	task: Task,
	filter: v.InferOutput<typeof TaskListQuery>,
) {
	if (filter.status && !filter.status.includes(task.status)) return false;
	if (filter.priority && !filter.priority.includes(task.priority)) return false;
	if (filter.milestone && task.milestoneId !== filter.milestone) return false;
	if (filter.dueBefore && (!task.dueOn || task.dueOn > filter.dueBefore))
		return false;
	if (filter.label) {
		const wanted = Array.isArray(filter.label) ? filter.label : [filter.label];
		if (!wanted.every((id) => task.labelIds?.includes(id))) return false;
	}
	return filter.includeSubtasks || !task.parentId;
}

export function emit(
	orgId: OrgId,
	event: WebhookEvent,
	payload: Record<string, unknown>,
) {
	const targets = [...db.webhooks.values()].filter(
		(hook) =>
			hook.orgId === orgId && hook.active && hook.events.includes(event),
	);
	return targets.map((hook) => ({ hookId: hook.id, event, payload }));
}
