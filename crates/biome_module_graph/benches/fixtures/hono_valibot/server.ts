import { Hono, HTTPException } from "./vendor/hono";
import * as v from "./vendor/valibot";
import {
	ActivityQuery,
	ArchiveProject,
	AuthHeaders,
	BulkTaskOperation,
	CommentParam,
	CreateComment,
	CreateLabel,
	CreateMilestone,
	CreateProject,
	CreateTask,
	CreateWebhook,
	IdempotencyHeaders,
	InviteMember,
	LabelParam,
	MemberParam,
	MilestoneParam,
	MoveTask,
	OptionalProjectParam,
	OrgParam,
	ProjectListQuery,
	ProjectParam,
	ReportQuery,
	TaskListQuery,
	TaskParam,
	UpdateComment,
	UpdateMember,
	UpdateProject,
	UpdateTask,
	WebhookParam,
	type Comment,
	type Label,
	type Member,
	type Milestone,
	type OrgId,
	type Project,
	type ProjectId,
	type Task,
	type TaskId,
	type TaskStatus,
	type UserId,
	type Webhook,
} from "./schema";
import {
	authenticate,
	compareBy,
	db,
	emit,
	errorBody,
	header,
	json,
	loadProject,
	newId,
	now,
	paginate,
	param,
	query,
	requestContext,
	requireFeature,
	requireRole,
	resolveTenant,
	taskMatches,
	type AppEnv,
} from "./context";

const taskComments = new Hono<AppEnv>()
	.get("/", param(TaskParam), (c) => {
		const { taskId } = c.req.valid("param");
		const comments = [...db.comments.values()].filter(
			(comment) => comment.taskId === taskId,
		);
		return c.json({ items: comments, total: comments.length }, 200);
	})
	.post("/", param(TaskParam), json(CreateComment), (c) => {
		const { orgId, taskId } = c.req.valid("param");
		const body = c.req.valid("json");
		if (!db.tasks.has(taskId)) {
			return c.json(errorBody(c, "task_not_found", taskId), 404);
		}
		const comment: Comment = {
			id: newId("cmt") as Comment["id"],
			taskId,
			authorId: c.var.user.id,
			body: body.body,
			mentions: body.mentions,
			replyTo: body.replyTo ?? null,
			editedAt: null,
			createdAt: now(),
		};
		db.comments.set(comment.id, comment);
		const deliveries = emit(orgId, "comment.created", { comment });
		return c.json({ comment, deliveries: deliveries.length }, 201);
	})
	.patch("/:commentId", param(CommentParam), json(UpdateComment), (c) => {
		const { commentId } = c.req.valid("param");
		const comment = db.comments.get(commentId);
		if (!comment) {
			return c.json(errorBody(c, "comment_not_found", commentId), 404);
		}
		if (comment.authorId !== c.var.user.id) {
			return c.json(
				errorBody(c, "not_author", "Only the author can edit"),
				403,
			);
		}
		const patch = c.req.valid("json");
		const updated = { ...comment, ...patch, editedAt: now() };
		db.comments.set(commentId, updated);
		return c.json(updated, 200);
	})
	.delete("/:commentId", param(CommentParam), (c) => {
		const { commentId } = c.req.valid("param");
		return db.comments.delete(commentId)
			? c.body(null, 204)
			: c.json(errorBody(c, "comment_not_found", commentId), 404);
	});

const tasks = new Hono<AppEnv>()
	.get("/", param(ProjectParam), query(TaskListQuery), (c) => {
		const { orgId, projectId } = c.req.valid("param");
		const filter = c.req.valid("query");
		if (!loadProject(orgId, projectId)) {
			return c.json(errorBody(c, "project_not_found", projectId), 404);
		}
		const sort = filter.sort ?? { field: "rank", direction: "asc" };
		const key = (
			{
				priority: "priority",
				status: "status",
				due_date: "dueOn",
				created_at: "createdAt",
				rank: "rank",
			} as const
		)[sort.field];
		const matching = [...db.tasks.values()]
			.filter((task) => task.projectId === projectId)
			.filter((task) => taskMatches(task, filter))
			.sort(compareBy(key, sort.direction));
		return c.json(paginate(matching, filter), 200);
	})
	.post(
		"/",
		param(ProjectParam),
		header(IdempotencyHeaders),
		json(CreateTask),
		(c) => {
			const { orgId, projectId } = c.req.valid("param");
			const input = c.req.valid("json");
			const project = loadProject(orgId, projectId);
			if (!project) {
				return c.json(errorBody(c, "project_not_found", projectId), 404);
			}
			if (
				input.kind === "bug" &&
				input.severity === "s1" &&
				!input.reproduction
			) {
				return c.json(
					errorBody(c, "reproduction_required", "S1 bugs need repro steps"),
					400,
				);
			}
			const task: Task = {
				...input,
				id: newId("tsk") as TaskId,
				projectId,
				number: project.taskCount + 1,
				status: input.status ?? project.settings.defaultStatus,
				priority: input.priority ?? project.settings.defaultPriority,
				reporterId: c.var.user.id,
				version: 0,
				rank: `0|${project.taskCount.toString(36)}`,
				closedAt: null,
				createdAt: now(),
				updatedAt: now(),
			};
			db.tasks.set(task.id, task);
			db.projects.set(projectId, { ...project, taskCount: task.number });
			emit(orgId, "task.created", { task });
			return c.json(task, 201);
		},
	)
	.post("/bulk", param(ProjectParam), json(BulkTaskOperation), (c) => {
		const { taskIds, operation } = c.req.valid("json");
		const results = taskIds.map((id) => {
			const task = db.tasks.get(id);
			if (!task)
				return { id, ok: false as const, reason: "not_found" as const };
			let next: Task;
			switch (operation.op) {
				case "move":
					next = { ...task, status: operation.status };
					break;
				case "assign":
					next = { ...task, assigneeIds: operation.assigneeIds };
					break;
				case "label":
					next = {
						...task,
						labelIds: [
							...(task.labelIds ?? []).filter(
								(label) => !operation.remove.includes(label),
							),
							...operation.add,
						],
					};
					break;
				case "prioritize":
					next = { ...task, priority: operation.priority };
					break;
				case "close":
					next = { ...task, status: "done", closedAt: now() };
					break;
			}
			db.tasks.set(id, { ...next, version: task.version + 1 });
			return { id, ok: true as const, version: task.version + 1 };
		});
		const failed = results.filter((result) => !result.ok);
		return failed.length === 0
			? c.json({ op: operation.op, results }, 200)
			: c.json({ op: operation.op, results, failed: failed.length }, 207);
	})
	.get("/:taskId", param(TaskParam), (c) => {
		const { taskId } = c.req.valid("param");
		const task = db.tasks.get(taskId);
		if (!task) return c.json(errorBody(c, "task_not_found", taskId), 404);
		const subtasks = [...db.tasks.values()].filter(
			(candidate) => candidate.parentId === taskId,
		);
		return c.json(
			{
				...task,
				subtasks: subtasks.map(({ id, title, status }) => ({
					id,
					title,
					status,
				})),
				commentCount: [...db.comments.values()].filter(
					(comment) => comment.taskId === taskId,
				).length,
			},
			200,
		);
	})
	.patch("/:taskId", param(TaskParam), json(UpdateTask), (c) => {
		const { orgId, taskId } = c.req.valid("param");
		const { version, ...patch } = c.req.valid("json");
		const task = db.tasks.get(taskId);
		if (!task) return c.json(errorBody(c, "task_not_found", taskId), 404);
		if (task.version !== version) {
			return c.json(
				{
					...errorBody(c, "version_conflict", "Task was modified"),
					current: task,
				},
				409,
			);
		}
		const updated: Task = {
			...task,
			...patch,
			status: patch.status ?? task.status,
			priority: patch.priority ?? task.priority,
			version: version + 1,
			updatedAt: now(),
		};
		db.tasks.set(taskId, updated);
		emit(orgId, "task.updated", { task: updated });
		return c.json(updated, 200);
	})
	.put("/:taskId/position", param(TaskParam), json(MoveTask), (c) => {
		const { orgId, taskId } = c.req.valid("param");
		const move = c.req.valid("json");
		const task = db.tasks.get(taskId);
		if (!task) return c.json(errorBody(c, "task_not_found", taskId), 404);
		if (move.projectId && !loadProject(orgId, move.projectId)) {
			return c.json(errorBody(c, "project_not_found", move.projectId), 404);
		}
		const before = move.beforeId ? db.tasks.get(move.beforeId) : undefined;
		const after = move.afterId ? db.tasks.get(move.afterId) : undefined;
		const rank = `${before?.rank ?? "0|"}${after ? "m" : "z"}`;
		const moved: Task = {
			...task,
			status: move.status,
			projectId: move.projectId ?? task.projectId,
			rank,
			version: task.version + 1,
		};
		db.tasks.set(taskId, moved);
		emit(orgId, "task.moved", { taskId, from: task.status, to: move.status });
		return c.json({ id: taskId, status: moved.status, rank }, 200);
	})
	.delete("/:taskId", param(TaskParam), requireRole("admin"), (c) => {
		const { taskId } = c.req.valid("param");
		if (!db.tasks.delete(taskId)) {
			return c.json(errorBody(c, "task_not_found", taskId), 404);
		}
		return c.body(null, 204);
	})
	.route("/:taskId/comments", taskComments);

const catalog = new Hono<AppEnv>()
	.get("/labels", param(ProjectParam), (c) => {
		const { projectId } = c.req.valid("param");
		const labels = [...db.labels.values()].filter(
			(label) => label.projectId === projectId,
		);
		return c.json(labels, 200);
	})
	.post("/labels", param(ProjectParam), json(CreateLabel), (c) => {
		const { projectId } = c.req.valid("param");
		const input = c.req.valid("json");
		const duplicate = [...db.labels.values()].some(
			(label) => label.projectId === projectId && label.name === input.name,
		);
		if (duplicate) {
			return c.json(errorBody(c, "label_exists", input.name), 409);
		}
		const label: Label = {
			id: newId("lbl") as Label["id"],
			projectId,
			name: input.name,
			color: input.color,
			description: input.description ?? null,
		};
		db.labels.set(label.id, label);
		return c.json(label, 201);
	})
	.delete("/labels/:labelId", param(LabelParam), (c) => {
		const { labelId } = c.req.valid("param");
		return db.labels.delete(labelId)
			? c.body(null, 204)
			: c.json(errorBody(c, "label_not_found", labelId), 404);
	})
	.get("/milestones", param(ProjectParam), (c) => {
		const { projectId } = c.req.valid("param");
		const milestones = [...db.milestones.values()]
			.filter((milestone) => milestone.projectId === projectId)
			.sort(compareBy("dueOn", "asc"));
		return c.json(
			{ open: milestones.filter((m) => m.state === "open"), milestones },
			200,
		);
	})
	.post("/milestones", param(ProjectParam), json(CreateMilestone), (c) => {
		const { projectId } = c.req.valid("param");
		const input = c.req.valid("json");
		const milestone: Milestone = {
			id: newId("mst") as Milestone["id"],
			projectId,
			title: input.title,
			dueOn: input.dueOn,
			state: input.state,
			progress: { total: 0, done: 0 },
		};
		db.milestones.set(milestone.id, milestone);
		return c.json(milestone, 201);
	})
	.patch(
		"/milestones/:milestoneId",
		param(MilestoneParam),
		json(v.partial(CreateMilestone)),
		(c) => {
			const { milestoneId } = c.req.valid("param");
			const milestone = db.milestones.get(milestoneId);
			if (!milestone) {
				return c.json(errorBody(c, "milestone_not_found", milestoneId), 404);
			}
			const updated = { ...milestone, ...c.req.valid("json") };
			db.milestones.set(milestoneId, updated);
			return c.json(updated, 200);
		},
	);

const projects = new Hono<AppEnv>()
	.get(
		"/:projectId?",
		param(OptionalProjectParam),
		query(ProjectListQuery),
		(c) => {
			const { orgId, projectId } = c.req.valid("param");
			if (projectId) {
				const project = loadProject(orgId, projectId);
				return project
					? c.json({ kind: "project" as const, project }, 200)
					: c.json(errorBody(c, "project_not_found", projectId), 404);
			}
			const filter = c.req.valid("query");
			const sort = filter.sort ?? { field: "updated_at", direction: "desc" };
			const key = (
				{
					name: "name",
					created_at: "createdAt",
					updated_at: "updatedAt",
					task_count: "taskCount",
				} as const
			)[sort.field];
			const matching = [...db.projects.values()]
				.filter((project) => project.orgId === orgId)
				.filter(
					(project) =>
						(filter.archived ?? false) === (project.archivedAt !== null),
				)
				.filter(
					(project) =>
						!filter.visibility ||
						filter.visibility.includes(project.visibility),
				)
				.filter(
					(project) =>
						!filter.search ||
						project.name.toLowerCase().includes(filter.search.toLowerCase()),
				)
				.sort(compareBy(key, sort.direction));
			return c.json(
				{ kind: "page" as const, ...paginate(matching, filter) },
				200,
			);
		},
	)
	.post(
		"/",
		param(OrgParam),
		header(IdempotencyHeaders),
		json(CreateProject),
		async (c) => {
			const { orgId } = c.req.valid("param");
			const key = c.req.valid("header")["idempotency-key"];
			const replay = db.idempotency.get(key);
			if (replay) {
				c.header("idempotent-replay", "true");
				return c.json(replay.body, replay.status);
			}
			const tenant = c.var.tenant;
			const count = [...db.projects.values()].filter(
				(project) => project.orgId === orgId,
			).length;
			if (count >= tenant.limits.projects) {
				return c.json(
					{
						...errorBody(c, "project_limit", "Project limit reached"),
						limit: tenant.limits.projects,
						plan: tenant.plan,
					},
					402,
				);
			}
			const input = c.req.valid("json");
			if ([...db.projects.values()].some((p) => p.slug === input.slug)) {
				return c.json(errorBody(c, "slug_taken", input.slug), 409);
			}
			const project: Project = {
				id: newId("prj") as ProjectId,
				orgId,
				name: input.name,
				slug: input.slug,
				description: input.description ?? null,
				visibility: input.visibility,
				color: input.color ?? null,
				leadId: input.leadId ?? null,
				memberIds: input.memberIds,
				settings: input.settings ?? {
					defaultStatus: "todo",
					defaultPriority: "none",
					allowGuests: false,
				},
				archivedAt: null,
				createdAt: now(),
				updatedAt: now(),
				taskCount: 0,
			};
			db.projects.set(project.id, project);
			db.idempotency.set(key, { status: 201, body: project });
			emit(orgId, "project.created", { project });
			return c.json(project, 201);
		},
	)
	.put("/:projectId", param(ProjectParam), json(CreateProject), (c) => {
		const { orgId, projectId } = c.req.valid("param");
		const existing = loadProject(orgId, projectId);
		if (!existing) {
			return c.json(errorBody(c, "project_not_found", projectId), 404);
		}
		const input = c.req.valid("json");
		const replaced: Project = {
			...existing,
			...input,
			description: input.description ?? null,
			color: input.color ?? null,
			leadId: input.leadId ?? null,
			settings: input.settings ?? existing.settings,
			updatedAt: now(),
		};
		db.projects.set(projectId, replaced);
		return c.json(replaced, 200);
	})
	.patch(
		"/:projectId",
		param(ProjectParam),
		header(AuthHeaders),
		json(UpdateProject),
		(c) => {
			const { orgId, projectId } = c.req.valid("param");
			const existing = loadProject(orgId, projectId);
			if (!existing) {
				return c.json(errorBody(c, "project_not_found", projectId), 404);
			}
			const ifMatch = c.req.valid("header")["if-match"];
			const etag = `W/"${Date.parse(existing.updatedAt)}"`;
			if (ifMatch && ifMatch !== etag) {
				return c.json(
					{ ...errorBody(c, "precondition_failed", "Stale"), etag },
					412,
				);
			}
			const patch = c.req.valid("json");
			const updated: Project = {
				...existing,
				...patch,
				description: patch.description ?? existing.description,
				color: patch.color ?? existing.color,
				leadId: patch.leadId ?? existing.leadId,
				settings: patch.settings ?? existing.settings,
				updatedAt: now(),
			};
			db.projects.set(projectId, updated);
			emit(orgId, "project.updated", {
				project: updated,
				fields: Object.keys(patch),
			});
			c.header("etag", `W/"${Date.parse(updated.updatedAt)}"`);
			return c.json(updated, 200);
		},
	)
	.delete("/:projectId", param(ProjectParam), requireRole("owner"), (c) => {
		const { projectId } = c.req.valid("param");
		if (!db.projects.delete(projectId)) {
			return c.json(errorBody(c, "project_not_found", projectId), 404);
		}
		return c.body(null, 204);
	})
	.post(
		"/:projectId/archive",
		param(ProjectParam),
		requireRole("admin"),
		json(ArchiveProject),
		(c) => {
			const { orgId, projectId } = c.req.valid("param");
			const { reason, mergeInto } = c.req.valid("json");
			const project = loadProject(orgId, projectId);
			if (!project) {
				return c.json(errorBody(c, "project_not_found", projectId), 404);
			}
			if (reason === "merged" && !mergeInto) {
				return c.json(errorBody(c, "merge_target_required", reason), 400);
			}
			const archived = { ...project, archivedAt: now() };
			db.projects.set(projectId, archived);
			emit(orgId, "project.archived", { projectId, reason });
			return c.json({ archived, reason: reason ?? "completed" }, 200);
		},
	)
	.get(
		"/:projectId/activity",
		param(ProjectParam),
		query(ActivityQuery),
		(c) => {
			const { projectId } = c.req.valid("param");
			const { limit, types, since } = c.req.valid("query");
			const events = [...db.tasks.values()]
				.filter((task) => task.projectId === projectId)
				.filter((task) => !since || task.updatedAt >= since)
				.slice(0, limit)
				.map((task) => ({
					type: task.closedAt
						? ("task.closed" as const)
						: ("task.updated" as const),
					taskId: task.id,
					at: task.updatedAt,
				}))
				.filter((event) => !types || types.includes(event.type));
			return c.json(
				{ projectId, events, truncated: events.length === limit },
				200,
			);
		},
	)
	.get(
		"/:projectId/report",
		param(ProjectParam),
		requireFeature("reports"),
		query(ReportQuery),
		(c) => {
			const { projectId } = c.req.valid("param");
			const report = c.req.valid("query");
			const counts = new Map<TaskStatus, number>();
			for (const task of db.tasks.values()) {
				if (task.projectId !== projectId) continue;
				counts.set(task.status, (counts.get(task.status) ?? 0) + 1);
			}
			const rows = [...counts].map(([status, count]) => ({ status, count }));
			if (report.format === "csv") {
				const csv = rows.map((row) => `${row.status},${row.count}`).join("\n");
				return c.text(`status,count\n${csv}`, 200, {
					"content-type": "text/csv",
				});
			}
			return c.json(
				{ projectId, range: { from: report.from, to: report.to }, rows },
				200,
			);
		},
	)
	.route("/:projectId/tasks", tasks)
	.route("/:projectId", catalog);

const members = new Hono<AppEnv>()
	.get("/", param(OrgParam), (c) => {
		const { orgId } = c.req.valid("param");
		const list = [...db.members.values()].filter((m) => m.orgId === orgId);
		return c.json(
			{
				owners: list.filter((m) => m.role === "owner").map((m) => m.userId),
				members: list,
			},
			200,
		);
	})
	.post("/", param(OrgParam), requireRole("admin"), json(InviteMember), (c) => {
		const { orgId } = c.req.valid("param");
		const invite = c.req.valid("json");
		if (invite.role === "guest" && !c.var.tenant.features.has("guests")) {
			return c.json(errorBody(c, "guests_disabled", invite.email), 402);
		}
		const member: Member = {
			userId: newId("usr") as UserId,
			orgId,
			email: invite.email,
			role: invite.role,
			projectIds: invite.projectIds,
			invitedAt: now(),
			joinedAt: null,
		};
		db.members.set(`${orgId}:${member.userId}`, member);
		emit(orgId, "member.invited", { email: member.email });
		return c.json({ member, inviteUrl: `/join/${member.userId}` }, 201);
	})
	.patch(
		"/:userId",
		param(MemberParam),
		requireRole("owner"),
		json(UpdateMember),
		(c) => {
			const { orgId, userId } = c.req.valid("param");
			const member = db.members.get(`${orgId}:${userId}`);
			if (!member) return c.json(errorBody(c, "member_not_found", userId), 404);
			if (member.role === "owner" && c.req.valid("json").role !== "owner") {
				const owners = [...db.members.values()].filter(
					(m) => m.orgId === orgId && m.role === "owner",
				);
				if (owners.length === 1) {
					return c.json(
						errorBody(c, "last_owner", "Promote another owner"),
						409,
					);
				}
			}
			const updated = { ...member, ...c.req.valid("json") };
			db.members.set(`${orgId}:${userId}`, updated);
			return c.json(updated, 200);
		},
	)
	.delete("/:userId", param(MemberParam), requireRole("admin"), (c) => {
		const { orgId, userId } = c.req.valid("param");
		return db.members.delete(`${orgId}:${userId}`)
			? c.body(null, 204)
			: c.json(errorBody(c, "member_not_found", userId), 404);
	});

const webhooks = new Hono<AppEnv>()
	.get("/", param(OrgParam), (c) => {
		const { orgId } = c.req.valid("param");
		return c.json(
			[...db.webhooks.values()].filter((hook) => hook.orgId === orgId),
			200,
		);
	})
	.post(
		"/",
		param(OrgParam),
		requireFeature("webhooks"),
		requireRole("admin"),
		json(CreateWebhook),
		(c) => {
			const { orgId } = c.req.valid("param");
			const input = c.req.valid("json");
			const hook: Webhook = {
				id: newId("whk") as Webhook["id"],
				orgId,
				url: input.url,
				events: input.events,
				active: input.active,
				secretLast4: input.secret?.slice(-4) ?? null,
				createdAt: now(),
			};
			db.webhooks.set(hook.id, hook);
			return c.json(hook, 201);
		},
	)
	.post("/:webhookId/test", param(WebhookParam), async (c) => {
		const { webhookId } = c.req.valid("param");
		const hook = db.webhooks.get(webhookId);
		if (!hook) return c.json(errorBody(c, "webhook_not_found", webhookId), 404);
		const response = await fetch(hook.url, {
			method: "POST",
			body: JSON.stringify({ event: "ping", hookId: hook.id }),
		}).catch(() => null);
		return response?.ok
			? c.json({ delivered: true as const, status: response.status }, 200)
			: c.json(
					{ delivered: false as const, status: response?.status ?? null },
					502,
				);
	})
	.delete("/:webhookId", param(WebhookParam), requireRole("admin"), (c) => {
		const { webhookId } = c.req.valid("param");
		return db.webhooks.delete(webhookId)
			? c.body(null, 204)
			: c.json(errorBody(c, "webhook_not_found", webhookId), 404);
	});

const orgs = new Hono<AppEnv>()
	.use("/:orgId/*", resolveTenant)
	.get("/:orgId", param(OrgParam), (c) => {
		const { tenant, role } = c.var;
		return c.json(
			{
				id: tenant.id,
				slug: tenant.slug,
				plan: tenant.plan,
				role,
				features: [...tenant.features],
				limits: tenant.limits,
			},
			200,
		);
	})
	.route("/:orgId/projects", projects)
	.route("/:orgId/members", members)
	.route("/:orgId/webhooks", webhooks);

const app = new Hono<AppEnv>()
	.basePath("/api/v1")
	.use(requestContext)
	.get("/health", (c) =>
		c.json({ status: "ok" as const, uptime: performance.now() }, 200),
	)
	.use(authenticate)
	.get("/me", (c) => {
		const { user } = c.var;
		return c.json(
			{
				id: user.id,
				email: user.email,
				name: user.name,
				orgs: Object.entries(user.roles).map(([orgId, role]) => ({
					orgId: orgId as OrgId,
					role,
				})),
			},
			200,
		);
	})
	.route("/orgs", orgs);

app.notFound((c) =>
	c.json(errorBody(c, "route_not_found", `${c.req.method} ${c.req.path}`), 404),
);

app.onError((error, c) => {
	if (error instanceof HTTPException) {
		return c.json(
			errorBody(c, `http_${error.status}`, error.message),
			error.status,
		);
	}
	console.error(error);
	return c.json(errorBody(c, "internal", "Unexpected error"), 500);
});

export type AppType = typeof app;
export type ProjectsRoutes = typeof projects;
export type TaskRoutes = typeof tasks;
export default app;
