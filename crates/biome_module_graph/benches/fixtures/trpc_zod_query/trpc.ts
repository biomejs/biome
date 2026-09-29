import {
	TRPCError,
	initTRPC,
	type inferProcedureBuilderResolverOptions,
} from "./vendor/trpc-server";
import * as z from "./vendor/zod";
import {
	ApiKey,
	AuditEntry,
	CreateApiKeyInput,
	CreateProjectInput,
	InviteInput,
	Invitation,
	Invoice,
	Member,
	Notification,
	Org,
	OrgSlug,
	PaymentMethod,
	Plan,
	Project,
	ProjectFilter,
	Role,
	Session,
	Task,
	TaskCommand,
	UpdateProjectInput,
	UsagePoint,
	UsageQuery,
	User,
	Webhook,
	roleRank,
} from "./schema";

type Row<T extends z.ZodType> = z.input<T>;
type Page<T> = { items: T[]; nextCursor: string | null; total: number };

export interface Db {
	users: {
		byId(id: string): Promise<Row<typeof User> | null>;
		update(
			id: string,
			patch: Partial<Pick<Row<typeof User>, "name" | "locale" | "avatarUrl">>,
		): Promise<Row<typeof User>>;
		sessions(userId: string): Promise<Row<typeof Session>[]>;
	};
	orgs: {
		bySlug(slug: string): Promise<Row<typeof Org> | null>;
		forUser(userId: string): Promise<Array<Row<typeof Org> & { role: Role }>>;
		insert(
			org: Omit<Row<typeof Org>, "id" | "createdAt">,
		): Promise<Row<typeof Org>>;
		updateSettings(
			orgId: string,
			settings: Partial<Row<typeof Org>["settings"]>,
		): Promise<Row<typeof Org>>;
	};
	memberships: {
		find(orgId: string, userId: string): Promise<{ role: Role } | null>;
		list(
			orgId: string,
			cursor: string | null,
			limit: number,
		): Promise<Page<Row<typeof Member>>>;
		setRole(
			orgId: string,
			userId: string,
			role: Role,
		): Promise<Row<typeof Member>>;
		remove(orgId: string, userId: string): Promise<void>;
		countOwners(orgId: string): Promise<number>;
	};
	invitations: {
		list(orgId: string): Promise<Row<typeof Invitation>[]>;
		create(
			orgId: string,
			invitedBy: string,
			input: z.output<typeof InviteInput>,
		): Promise<Row<typeof Invitation>>;
		revoke(orgId: string, id: string): Promise<boolean>;
	};
	projects: {
		list(
			orgId: string,
			filter: ProjectFilter,
		): Promise<Page<Row<typeof Project>>>;
		byId(orgId: string, id: string): Promise<Row<typeof Project> | null>;
		insert(
			orgId: string,
			ownerId: string,
			input: z.output<typeof CreateProjectInput>,
		): Promise<Row<typeof Project>>;
		update(
			orgId: string,
			patch: z.output<typeof UpdateProjectInput>,
		): Promise<Row<typeof Project>>;
	};
	tasks: {
		list(
			projectId: string,
			opts: { cursor: string | null; limit: number; assigneeId?: string },
		): Promise<Page<Row<typeof Task>>>;
		byNumber(
			projectId: string,
			number: number,
		): Promise<Row<typeof Task> | null>;
		apply(command: TaskCommand, actorId: string): Promise<Row<typeof Task>>;
	};
	billing: {
		subscription(orgId: string): Promise<{
			plan: Plan;
			seats: number;
			interval: "month" | "year";
			renewsAt: string;
			cancelAtPeriodEnd: boolean;
		}>;
		invoices(
			orgId: string,
			cursor: string | null,
			limit: number,
		): Promise<Page<Row<typeof Invoice>>>;
		invoice(orgId: string, id: string): Promise<Row<typeof Invoice> | null>;
		paymentMethods(
			orgId: string,
		): Promise<
			Array<Row<typeof PaymentMethod> & { id: string; isDefault: boolean }>
		>;
		usage(
			orgId: string,
			query: z.output<typeof UsageQuery>,
		): Promise<Row<typeof UsagePoint>[]>;
	};
	apiKeys: {
		list(orgId: string): Promise<Row<typeof ApiKey>[]>;
		create(
			orgId: string,
			input: z.output<typeof CreateApiKeyInput>,
		): Promise<{ key: Row<typeof ApiKey>; secret: string }>;
		revoke(orgId: string, id: string): Promise<void>;
	};
	webhooks: {
		list(orgId: string): Promise<Row<typeof Webhook>[]>;
		upsert(
			orgId: string,
			hook: Omit<Row<typeof Webhook>, "id"> & { id?: string },
		): Promise<Row<typeof Webhook>>;
	};
	audit: {
		list(
			orgId: string,
			cursor: string | null,
			limit: number,
		): Promise<Page<Row<typeof AuditEntry>>>;
		record(
			orgId: string,
			actorId: string,
			event: AuditEntry["event"],
		): Promise<void>;
	};
	notifications: {
		list(
			userId: string,
			cursor: string | null,
			limit: number,
		): Promise<Page<Row<typeof Notification>>>;
		markRead(userId: string, ids: string[]): Promise<number>;
		subscribe(
			userId: string,
			signal: AbortSignal | undefined,
		): AsyncIterable<Row<typeof Notification>>;
	};
}

type AuditEntry = z.infer<typeof AuditEntry>;

export interface Context {
	db: Db;
	session: z.infer<typeof Session> | null;
	requestId: string;
	ip: string | null;
	userAgent: string | null;
}

export interface Meta {
	requiredRole?: Role;
	rateLimit?: { windowMs: number; max: number };
	audit?: boolean;
	description?: string;
}

export async function createContext(opts: {
	req: Request;
	db: Db;
	resolveSession: (token: string | null) => Promise<Context["session"]>;
}): Promise<Context> {
	const token =
		opts.req.headers.get("authorization")?.replace(/^Bearer /, "") ?? null;
	return {
		db: opts.db,
		session: await opts.resolveSession(token),
		requestId: opts.req.headers.get("x-request-id") ?? crypto.randomUUID(),
		ip: opts.req.headers.get("x-forwarded-for"),
		userAgent: opts.req.headers.get("user-agent"),
	};
}

const t = initTRPC
	.context<Context>()
	.meta<Meta>()
	.create({
		errorFormatter({ shape, error }) {
			return {
				...shape,
				data: {
					...shape.data,
					zodError:
						error.cause instanceof z.ZodError
							? z.flattenError(error.cause)
							: null,
					retryable:
						error.code === "TOO_MANY_REQUESTS" || error.code === "TIMEOUT",
				},
			};
		},
		defaultMeta: { audit: false },
	});

const buckets = new Map<string, { count: number; resetAt: number }>();

const timing = t.middleware(async ({ path, type, next, ctx }) => {
	const start = performance.now();
	const result = await next({ ctx: { startedAt: start } });
	const elapsed = performance.now() - start;
	if (!result.ok)
		console.warn(
			`[${ctx.requestId}] ${type} ${path} failed after ${elapsed}ms`,
		);
	return result;
});

const rateLimit = t.middleware(async ({ meta, ctx, path, next }) => {
	if (!meta?.rateLimit) return next();
	const key = `${ctx.session?.userId ?? ctx.ip}:${path}`;
	const now = Date.now();
	const bucket = buckets.get(key) ?? {
		count: 0,
		resetAt: now + meta.rateLimit.windowMs,
	};
	if (bucket.resetAt < now) {
		bucket.count = 0;
		bucket.resetAt = now + meta.rateLimit.windowMs;
	}
	bucket.count += 1;
	buckets.set(key, bucket);
	if (bucket.count > meta.rateLimit.max) {
		throw new TRPCError({
			code: "TOO_MANY_REQUESTS",
			message: `Retry after ${Math.ceil((bucket.resetAt - now) / 1000)}s`,
		});
	}
	return next();
});

const isAuthed = t.middleware(async ({ ctx, next }) => {
	if (!ctx.session) throw new TRPCError({ code: "UNAUTHORIZED" });
	const user = await ctx.db.users.byId(ctx.session.userId);
	if (!user)
		throw new TRPCError({
			code: "UNAUTHORIZED",
			message: "User no longer exists",
		});
	return next({
		ctx: {
			session: ctx.session,
			user: User.parse(user),
			impersonating: ctx.session.impersonatorId !== undefined,
		},
	});
});

export const publicProcedure = t.procedure.use(timing).use(rateLimit);
export const protectedProcedure = publicProcedure.use(isAuthed);

export const orgProcedure = protectedProcedure
	.input(z.object({ orgSlug: OrgSlug }))
	.use(async ({ ctx, input, meta, next }) => {
		const org = await ctx.db.orgs.bySlug(input.orgSlug);
		if (!org)
			throw new TRPCError({
				code: "NOT_FOUND",
				message: `Unknown org ${input.orgSlug}`,
			});
		const membership = await ctx.db.memberships.find(org.id, ctx.user.id);
		if (!membership) throw new TRPCError({ code: "FORBIDDEN" });
		const required = meta?.requiredRole ?? "viewer";
		if (roleRank[membership.role] < roleRank[required]) {
			throw new TRPCError({
				code: "FORBIDDEN",
				message: `Requires ${required}, you are ${membership.role}`,
			});
		}
		const parsed = Org.parse(org);
		if (parsed.settings.requireTwoFactor && !ctx.user.twoFactorEnabled) {
			throw new TRPCError({
				code: "PRECONDITION_FAILED",
				message: "Two-factor required",
			});
		}
		return next({
			ctx: {
				org: { ...parsed, role: membership.role },
				can: (role: Role) => roleRank[membership.role] >= roleRank[role],
			},
		});
	});

export const adminProcedure = orgProcedure
	.meta({ requiredRole: "admin" })
	.use(({ ctx, next }) => {
		if (ctx.org.role !== "admin" && ctx.org.role !== "owner") {
			throw new TRPCError({ code: "FORBIDDEN" });
		}
		return next({
			ctx: { org: { ...ctx.org, role: ctx.org.role }, admin: ctx.user },
		});
	});

export const billingProcedure = orgProcedure.use(({ ctx, next }) => {
	const { role } = ctx.org;
	if (role !== "owner" && role !== "billing" && role !== "admin") {
		throw new TRPCError({
			code: "FORBIDDEN",
			message: "Billing access required",
		});
	}
	return next({ ctx: { billingRole: role } });
});

export const audited = adminProcedure.meta({
	requiredRole: "admin",
	audit: true,
});

type AdminOpts = inferProcedureBuilderResolverOptions<typeof adminProcedure>;

export async function recordAudit(
	opts: Pick<AdminOpts, "ctx">,
	event: AuditEntry["event"],
) {
	await opts.ctx.db.audit.record(opts.ctx.org.id, opts.ctx.admin.id, event);
}

export const router = t.router;
export const createCallerFactory = t.createCallerFactory;
