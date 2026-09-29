import { TRPCError, tracked } from "./vendor/trpc-server";
import * as z from "./vendor/zod";
import {
	ApiKey,
	AuditEntry,
	ChangePlanInput,
	CreateApiKeyInput,
	CreateProjectInput,
	Id,
	InviteInput,
	Invitation,
	Invoice,
	InvoiceSummary,
	Member,
	Notification,
	Org,
	Pagination,
	PaymentMethod,
	Plan,
	Project,
	ProjectFilter,
	PublicUser,
	Role,
	Session,
	Task,
	TaskCommand,
	UpdateMemberRoleInput,
	UpdateProjectInput,
	UsagePoint,
	UsageQuery,
	User,
	Webhook,
	WebhookEvent,
	page,
} from "./schema";
import {
	adminProcedure,
	audited,
	billingProcedure,
	createCallerFactory,
	orgProcedure,
	protectedProcedure,
	publicProcedure,
	recordAudit,
	router,
	type Context,
} from "./trpc";

const byId = z.object({ id: Id });

const viewerRouter = router({
	me: protectedProcedure
		.output(
			User.extend({
				orgs: z.array(
					Org.pick({ slug: true, name: true, plan: true }).extend({
						role: Role,
					}),
				),
			}),
		)
		.query(async ({ ctx }) => {
			const orgs = await ctx.db.orgs.forUser(ctx.user.id);
			return {
				...ctx.user,
				createdAt: ctx.user.createdAt.toISOString(),
				orgs: orgs.map(({ slug, name, plan, role }) => ({
					slug,
					name,
					plan,
					role,
				})),
			};
		}),
	sessions: protectedProcedure.query(async ({ ctx }) => {
		const sessions = await ctx.db.users.sessions(ctx.user.id);
		return sessions.map((session) => ({
			...Session.parse(session),
			current: session.id === ctx.session.id,
		}));
	}),
	updateProfile: protectedProcedure
		.meta({ rateLimit: { windowMs: 60_000, max: 5 } })
		.input(User.pick({ name: true, locale: true, avatarUrl: true }).partial())
		.mutation(async ({ ctx, input }) =>
			User.parse(await ctx.db.users.update(ctx.user.id, input)),
		),
	notifications: protectedProcedure
		.input(Pagination.extend({ unreadOnly: z.boolean().default(false) }))
		.output(page(Notification))
		.query(async ({ ctx, input }) => {
			const result = await ctx.db.notifications.list(
				ctx.user.id,
				input.cursor ?? null,
				input.limit,
			);
			return input.unreadOnly
				? { ...result, items: result.items.filter((n) => !n.read) }
				: result;
		}),
	markRead: protectedProcedure
		.input(z.object({ ids: z.array(Id).min(1).max(100) }))
		.mutation(async ({ ctx, input }) => ({
			updated: await ctx.db.notifications.markRead(ctx.user.id, input.ids),
		})),
	onNotification: protectedProcedure
		.input(z.object({ lastEventId: z.string().nullish() }).optional())
		.subscription(async function* ({ ctx, signal }) {
			for await (const raw of ctx.db.notifications.subscribe(
				ctx.user.id,
				signal,
			)) {
				const notification = Notification.parse(raw);
				yield tracked(notification.id, notification);
			}
		}),
});

const membersRouter = router({
	list: orgProcedure
		.input(Pagination.extend({ role: Role.optional() }))
		.output(page(Member))
		.query(async ({ ctx, input }) => {
			const result = await ctx.db.memberships.list(
				ctx.org.id,
				input.cursor ?? null,
				input.limit,
			);
			return input.role
				? {
						...result,
						items: result.items.filter((m) => m.role === input.role),
					}
				: result;
		}),
	invitations: adminProcedure
		.output(z.array(Invitation))
		.query(({ ctx }) => ctx.db.invitations.list(ctx.org.id)),
	invite: audited
		.meta({
			requiredRole: "admin",
			audit: true,
			rateLimit: { windowMs: 3_600_000, max: 50 },
		})
		.input(InviteInput)
		.mutation(async (opts) => {
			const { ctx, input } = opts;
			const domain = input.email.split("@")[1];
			const allowed = ctx.org.settings.allowedDomains;
			if (allowed.length > 0 && domain && !allowed.includes(domain)) {
				throw new TRPCError({
					code: "BAD_REQUEST",
					message: `${domain} is not an allowed domain`,
				});
			}
			const invitation = Invitation.parse(
				await ctx.db.invitations.create(ctx.org.id, ctx.admin.id, input),
			);
			await recordAudit(opts, {
				action: "member.invited",
				email: input.email,
				role: input.role,
			});
			return invitation;
		}),
	revokeInvite: adminProcedure.input(byId).mutation(async ({ ctx, input }) => {
		const revoked = await ctx.db.invitations.revoke(ctx.org.id, input.id);
		if (!revoked) throw new TRPCError({ code: "NOT_FOUND" });
		return { id: input.id, revoked };
	}),
	updateRole: audited.input(UpdateMemberRoleInput).mutation(async (opts) => {
		const { ctx, input } = opts;
		if (input.role === "owner" && ctx.org.role !== "owner") {
			throw new TRPCError({
				code: "FORBIDDEN",
				message: "Only owners can transfer ownership",
			});
		}
		const current = await ctx.db.memberships.find(ctx.org.id, input.userId);
		if (!current) throw new TRPCError({ code: "NOT_FOUND" });
		const member = Member.parse(
			await ctx.db.memberships.setRole(ctx.org.id, input.userId, input.role),
		);
		await recordAudit(opts, {
			action: "member.role_changed",
			userId: member.userId,
			from: current.role,
			to: member.role,
		});
		return member;
	}),
	remove: adminProcedure
		.input(z.object({ userId: Id }))
		.mutation(async ({ ctx, input }) => {
			if (
				input.userId === ctx.admin.id &&
				ctx.org.role === "owner" &&
				(await ctx.db.memberships.countOwners(ctx.org.id)) <= 1
			) {
				throw new TRPCError({
					code: "CONFLICT",
					message: "Cannot remove the last owner",
				});
			}
			await ctx.db.memberships.remove(ctx.org.id, input.userId);
			return { removed: input.userId };
		}),
});

const tasksRouter = router({
	list: orgProcedure
		.input(
			Pagination.extend({
				projectId: Id,
				assignee: z.union([z.literal("me"), Id]).optional(),
			}),
		)
		.output(page(Task))
		.query(({ ctx, input }) =>
			ctx.db.tasks.list(input.projectId, {
				cursor: input.cursor ?? null,
				limit: input.limit,
				assigneeId: input.assignee === "me" ? ctx.user.id : input.assignee,
			}),
		),
	byNumber: orgProcedure
		.input(
			z.object({ projectId: Id, number: z.coerce.number().int().positive() }),
		)
		.output(Task)
		.query(async ({ ctx, input }) => {
			const task = await ctx.db.tasks.byNumber(input.projectId, input.number);
			if (!task) throw new TRPCError({ code: "NOT_FOUND" });
			return task;
		}),
	board: orgProcedure
		.input(z.object({ projectId: Id }))
		.query(async ({ ctx, input }) => {
			const { items } = await ctx.db.tasks.list(input.projectId, {
				cursor: null,
				limit: 500,
			});
			const tasks = items.map((row) => Task.parse(row));
			return {
				todo: tasks.filter((task) => task.state.kind === "todo"),
				inProgress: tasks.filter((task) => task.state.kind === "in_progress"),
				blocked: tasks.flatMap((task) =>
					task.state.kind === "blocked"
						? [{ task, blockedBy: task.state.blockedBy }]
						: [],
				),
				done: tasks.flatMap((task) =>
					task.state.kind === "done"
						? [{ id: task.id, resolution: task.state.resolution }]
						: [],
				),
			};
		}),
	command: orgProcedure
		.meta({ requiredRole: "member" })
		.input(TaskCommand)
		.mutation(async ({ ctx, input }) => {
			const task = Task.parse(await ctx.db.tasks.apply(input, ctx.user.id));
			switch (input.type) {
				case "assign":
					return { type: input.type, task, assigneeId: input.assigneeId };
				case "move":
					return {
						type: input.type,
						task,
						from: task.state.kind,
						to: input.to,
					};
				case "block":
					return { type: input.type, task, blockers: input.blockedBy.length };
				case "relabel":
					return { type: input.type, task, labels: task.labels };
			}
		}),
});

const projectsRouter = router({
	list: orgProcedure
		.input(ProjectFilter)
		.output(page(Project))
		.query(({ ctx, input }) => ctx.db.projects.list(ctx.org.id, input)),
	byId: orgProcedure
		.input(byId)
		.output(Project)
		.query(async ({ ctx, input }) => {
			const project = await ctx.db.projects.byId(ctx.org.id, input.id);
			if (!project)
				throw new TRPCError({
					code: "NOT_FOUND",
					message: `Project ${input.id} not found`,
				});
			return project;
		}),
	create: orgProcedure
		.meta({ requiredRole: "member" })
		.input(CreateProjectInput)
		.mutation(async ({ ctx, input }) => {
			const limits = {
				free: 3,
				starter: 20,
				team: 200,
				enterprise: Infinity,
			} satisfies Record<Plan, number>;
			const { total } = await ctx.db.projects.list(
				ctx.org.id,
				ProjectFilter.parse({ limit: 1 }),
			);
			if (total >= limits[ctx.org.plan]) {
				throw new TRPCError({
					code: "FORBIDDEN",
					message: `Plan ${ctx.org.plan} allows ${limits[ctx.org.plan]} projects`,
				});
			}
			return Project.parse(
				await ctx.db.projects.insert(ctx.org.id, ctx.user.id, input),
			);
		}),
	update: orgProcedure
		.meta({ requiredRole: "member" })
		.input(UpdateProjectInput)
		.mutation(async ({ ctx, input }) =>
			Project.parse(await ctx.db.projects.update(ctx.org.id, input)),
		),
	archive: adminProcedure.input(byId).mutation(async ({ ctx, input }) => {
		const project = Project.parse(
			await ctx.db.projects.update(ctx.org.id, {
				id: input.id,
				status: "archived",
			}),
		);
		return {
			id: project.id,
			status: project.status,
			archivedBy: ctx.admin.name,
		};
	}),
	tasks: tasksRouter,
});

const settingsRouter = router({
	apiKeys: router({
		list: adminProcedure
			.output(z.array(ApiKey))
			.query(({ ctx }) => ctx.db.apiKeys.list(ctx.org.id)),
		create: audited.input(CreateApiKeyInput).mutation(async (opts) => {
			const { key, secret } = await opts.ctx.db.apiKeys.create(
				opts.ctx.org.id,
				opts.input,
			);
			const parsed = ApiKey.parse(key);
			await recordAudit(opts, {
				action: "apikey.created",
				keyId: parsed.id,
				scopes: parsed.scopes,
			});
			return { key: parsed, secret, shownOnce: true as const };
		}),
		revoke: adminProcedure
			.input(byId)
			.mutation(({ ctx, input }) =>
				ctx.db.apiKeys.revoke(ctx.org.id, input.id),
			),
	}),
	webhooks: router({
		list: adminProcedure.query(async ({ ctx }) =>
			(await ctx.db.webhooks.list(ctx.org.id)).map((hook) =>
				Webhook.omit({ secret: true }).parse(hook),
			),
		),
		upsert: adminProcedure
			.input(
				Webhook.omit({ id: true, secret: true }).extend({
					id: Id.optional(),
					rotateSecret: z.boolean().default(false),
				}),
			)
			.mutation(async ({ ctx, input }) => {
				const { rotateSecret, ...hook } = input;
				const secret =
					rotateSecret || !hook.id
						? crypto.randomUUID().replaceAll("-", "")
						: "unchanged-secret";
				return Webhook.parse(
					await ctx.db.webhooks.upsert(ctx.org.id, { ...hook, secret }),
				);
			}),
		test: adminProcedure
			.meta({ requiredRole: "admin", rateLimit: { windowMs: 60_000, max: 3 } })
			.input(z.object({ id: Id, event: WebhookEvent }))
			.mutation(({ input }) => ({
				delivered: true,
				id: input.id,
				event: input.event,
				status: 200 as number,
			})),
	}),
	update: audited
		.input(
			Org.shape.settings.partial().extend({ name: Org.shape.name.optional() }),
		)
		.mutation(async ({ ctx, input }) => {
			const { name: _name, ...settings } = input;
			return Org.parse(await ctx.db.orgs.updateSettings(ctx.org.id, settings));
		}),
	audit: audited
		.input(Pagination)
		.output(page(AuditEntry))
		.query(({ ctx, input }) =>
			ctx.db.audit.list(ctx.org.id, input.cursor ?? null, input.limit),
		),
});

const orgRouter = router({
	list: protectedProcedure.query(async ({ ctx }) => {
		const orgs = await ctx.db.orgs.forUser(ctx.user.id);
		return orgs.map((org) => ({ ...Org.parse(org), role: org.role }));
	}),
	create: protectedProcedure
		.input(
			Org.pick({ slug: true, name: true }).extend({
				plan: Plan.default("free"),
			}),
		)
		.mutation(async ({ ctx, input }) => {
			const existing = await ctx.db.orgs.bySlug(input.slug);
			if (existing)
				throw new TRPCError({
					code: "CONFLICT",
					message: `${input.slug} is taken`,
				});
			const org = await ctx.db.orgs.insert({
				...input,
				seats: 1,
				settings: {
					allowedDomains: [],
					requireTwoFactor: false,
					defaultRole: "member",
				},
			});
			return Org.parse(org);
		}),
	bySlug: orgProcedure.query(({ ctx }) => ({
		...ctx.org,
		permissions: {
			manageMembers: ctx.can("admin"),
			manageBilling: ctx.can("billing") && ctx.org.role !== "member",
			createProjects: ctx.can("member"),
		},
	})),
	members: membersRouter,
	projects: projectsRouter,
	settings: settingsRouter,
});

const billingRouter = router({
	subscription: router({
		current: billingProcedure.query(async ({ ctx }) => {
			const sub = await ctx.db.billing.subscription(ctx.org.id);
			return {
				...sub,
				renewsAt: new Date(sub.renewsAt),
				viewerRole: ctx.billingRole,
			};
		}),
		changePlan: billingProcedure
			.meta({ requiredRole: "billing", audit: true })
			.input(ChangePlanInput)
			.mutation(async ({ ctx, input }) => {
				const current = await ctx.db.billing.subscription(ctx.org.id);
				if (input.seats < ctx.org.seats) {
					throw new TRPCError({
						code: "BAD_REQUEST",
						message: "Remove members before reducing seats",
					});
				}
				await ctx.db.audit.record(ctx.org.id, ctx.user.id, {
					action: "billing.plan_changed",
					from: current.plan,
					to: input.plan,
					seats: input.seats,
				});
				return {
					previous: current.plan,
					next: input.plan,
					prorated: input.interval === "year",
				};
			}),
		cancel: billingProcedure
			.input(
				z.object({
					reason: z.enum(["price", "features", "switching", "other"]),
					feedback: z.string().max(1000).optional(),
				}),
			)
			.mutation(({ input }) => ({
				cancelAtPeriodEnd: true as const,
				reason: input.reason,
			})),
	}),
	invoices: router({
		list: billingProcedure
			.input(Pagination.extend({ status: Invoice.shape.status.optional() }))
			.output(page(InvoiceSummary))
			.query(({ ctx, input }) =>
				ctx.db.billing.invoices(ctx.org.id, input.cursor ?? null, input.limit),
			),
		byId: billingProcedure
			.input(byId)
			.output(Invoice)
			.query(async ({ ctx, input }) => {
				const invoice = await ctx.db.billing.invoice(ctx.org.id, input.id);
				if (!invoice) throw new TRPCError({ code: "NOT_FOUND" });
				return invoice;
			}),
		download: billingProcedure
			.input(byId.extend({ format: z.enum(["pdf", "csv"]).default("pdf") }))
			.mutation(({ ctx, input }) => ({
				url: new URL(
					`/orgs/${ctx.org.slug}/invoices/${input.id}.${input.format}`,
					"https://billing.example.com",
				).href,
				expiresAt: new Date(Date.now() + 300_000),
			})),
	}),
	paymentMethods: router({
		list: billingProcedure.query(async ({ ctx }) =>
			(await ctx.db.billing.paymentMethods(ctx.org.id)).map(
				({ id, isDefault, ...method }) => ({
					id,
					isDefault,
					method: PaymentMethod.parse(method),
				}),
			),
		),
		setDefault: billingProcedure
			.input(byId)
			.mutation(({ input }) => ({ defaultId: input.id })),
	}),
	usage: billingProcedure.input(UsageQuery).query(async ({ ctx, input }) => {
		const points = (await ctx.db.billing.usage(ctx.org.id, input)).map((p) =>
			UsagePoint.parse(p),
		);
		const totals = Object.fromEntries(
			input.metrics.map((metric) => [
				metric,
				points
					.filter((p) => p.metric === metric)
					.reduce((sum, p) => sum + p.value, 0),
			]),
		) as Record<(typeof input.metrics)[number], number>;
		return { points, totals, granularity: input.granularity };
	}),
});

export const appRouter = router({
	health: publicProcedure.query(() => ({
		status: "ok" as const,
		time: new Date(),
	})),
	viewer: viewerRouter,
	org: orgRouter,
	billing: billingRouter,
	lookupUser: protectedProcedure
		.input(z.object({ id: Id }))
		.output(PublicUser.nullable())
		.query(async ({ ctx, input }) => {
			const user = await ctx.db.users.byId(input.id);
			return (
				user && { id: user.id, name: user.name, avatarUrl: user.avatarUrl }
			);
		}),
});

export type AppRouter = typeof appRouter;

export const createCaller = createCallerFactory(appRouter);

export async function seedDemo(ctx: Context) {
	const caller = createCaller(ctx);
	const me = await caller.viewer.me();
	const org = await caller.org.create({ slug: "acme", name: "Acme Corp" });
	const project = await caller.org.projects.create({
		orgSlug: org.slug,
		key: "WEB",
		name: "Website",
		description: null,
		visibility: "org",
		tags: ["marketing"],
	});
	await caller.org.members.invite({
		orgSlug: org.slug,
		email: "new@acme.test",
		role: "member",
	});
	const board = await caller.org.projects.tasks.board({
		orgSlug: org.slug,
		projectId: project.id,
	});
	return {
		me: me.email,
		org: org.id,
		project: project.key,
		blocked: board.blocked.length,
	};
}
