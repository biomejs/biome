import {
	TRPCClientError,
	createTRPCClient,
	httpBatchLink,
	httpLink,
	isTRPCClientError,
	loggerLink,
	splitLink,
	type TRPCLink,
} from "./vendor/trpc-client";
import {
	observable,
	type inferRouterInputs,
	type inferRouterOutputs,
} from "./vendor/trpc-server";
import {
	InfiniteQueryObserver,
	MutationObserver,
	QueryClient,
	QueryObserver,
	hashKey,
	keepPreviousData,
	skipToken,
	type InfiniteData,
} from "./vendor/tanstack-query";
import {
	createTRPCOptionsProxy,
	type TRPCOptionsProxy,
	type inferInput,
	type inferOutput,
} from "./vendor/trpc-tanstack-react-query";
import type { AppRouter } from "./router";
import { Id, roleRank, type Role } from "./schema";

export type RouterInputs = inferRouterInputs<AppRouter>;
export type RouterOutputs = inferRouterOutputs<AppRouter>;

export type Viewer = RouterOutputs["viewer"]["me"];
export type OrgDetails = RouterOutputs["org"]["bySlug"];
export type ProjectPage = RouterOutputs["org"]["projects"]["list"];
export type ProjectRow = ProjectPage["items"][number];
export type TaskRow =
	RouterOutputs["org"]["projects"]["tasks"]["list"]["items"][number];
export type Board = RouterOutputs["org"]["projects"]["tasks"]["board"];
export type TaskCommandResult =
	RouterOutputs["org"]["projects"]["tasks"]["command"];
export type InvoiceDetail = RouterOutputs["billing"]["invoices"]["byId"];
export type Usage = RouterOutputs["billing"]["usage"];
export type AuditRow =
	RouterOutputs["org"]["settings"]["audit"]["items"][number];
export type ProjectListInput = RouterInputs["org"]["projects"]["list"];
export type InviteMemberInput = RouterInputs["org"]["members"]["invite"];
export type AppClientError = TRPCClientError<AppRouter>;

const API_URL = "https://api.example.com/trpc";
let accessToken: string | null = null;

export function setAccessToken(token: string | null) {
	accessToken = token;
}

const refreshLink: TRPCLink<AppRouter> = () => {
	return ({ op, next }) =>
		observable((observer) => {
			let attempts = 0;
			let subscription: { unsubscribe(): void } | undefined;
			const attempt = () => {
				attempts += 1;
				subscription = next(op).subscribe({
					next: (value) => observer.next(value),
					error: (error) => {
						if (
							error.data?.code === "UNAUTHORIZED" &&
							attempts < 2 &&
							op.type !== "subscription"
						) {
							accessToken = null;
							attempt();
							return;
						}
						observer.error(error);
					},
					complete: () => observer.complete(),
				});
			};
			attempt();
			return () => subscription?.unsubscribe();
		});
};

export const client = createTRPCClient<AppRouter>({
	links: [
		loggerLink({
			enabled: (opts) =>
				opts.direction === "down" && opts.result instanceof Error,
		}),
		refreshLink,
		splitLink({
			condition: (op) =>
				op.context.skipBatch === true ||
				op.path.startsWith("billing.invoices.download"),
			true: httpLink({
				url: API_URL,
				headers: () =>
					accessToken ? { authorization: `Bearer ${accessToken}` } : {},
			}),
			false: httpBatchLink({
				url: API_URL,
				maxURLLength: 2083,
				maxItems: 10,
				headers: ({ opList }) => ({
					...(accessToken ? { authorization: `Bearer ${accessToken}` } : {}),
					"x-batch-size": String(opList.length),
				}),
			}),
		}),
	],
});

export const queryClient = new QueryClient({
	defaultOptions: {
		queries: {
			staleTime: 30_000,
			retry: (failureCount, error) => {
				if (isTRPCClientError<AppRouter>(error)) {
					if (error.data?.retryable) return failureCount < 5;
					const status = error.data?.httpStatus ?? 500;
					return status >= 500 && failureCount < 3;
				}
				return failureCount < 1;
			},
		},
		mutations: { retry: false },
	},
});

export const trpc: TRPCOptionsProxy<AppRouter> =
	createTRPCOptionsProxy<AppRouter>({
		client,
		queryClient,
	});

type ProjectsProc = typeof trpc.org.projects.list;
export type ProjectsInput = inferInput<ProjectsProc>;
export type ProjectsOutput = inferOutput<ProjectsProc>;

export function fieldErrors(error: unknown): Record<string, string[]> {
	if (!(error instanceof TRPCClientError)) return {};
	const typed = error as AppClientError;
	const zodError = typed.data?.zodError;
	if (!zodError) return {};
	const entries = Object.entries(zodError.fieldErrors) as Array<
		[string, string[] | undefined]
	>;
	return Object.fromEntries(
		entries.filter(
			(entry): entry is [string, string[]] => entry[1] !== undefined,
		),
	);
}

export interface ProjectCard {
	id: string;
	key: string;
	title: string;
	subtitle: string;
	status: ProjectRow["status"];
	badge: "public" | "internal" | null;
	progress: number;
	updated: Date;
}

export function toProjectCard(row: ProjectRow): ProjectCard {
	const total = row.stats.openTasks + row.stats.closedTasks;
	return {
		id: row.id,
		key: row.key,
		title: row.name,
		subtitle: row.description ?? `${row.tags.length} tags`,
		status: row.status,
		badge:
			row.visibility === "public"
				? "public"
				: row.visibility === "org"
					? "internal"
					: null,
		progress: total === 0 ? 0 : row.stats.closedTasks / total,
		updated: new Date(row.updatedAt),
	};
}

export async function loadDashboard(orgSlug: string) {
	const [viewer, org, projects, subscription] = await Promise.all([
		queryClient.fetchQuery(trpc.viewer.me.queryOptions()),
		queryClient.ensureQueryData(trpc.org.bySlug.queryOptions({ orgSlug })),
		queryClient.fetchInfiniteQuery(
			trpc.org.projects.list.infiniteQueryOptions(
				{ orgSlug, limit: 20, status: ["active", "paused"] },
				{ getNextPageParam: (last) => last.nextCursor },
			),
		),
		queryClient.fetchQuery(
			trpc.billing.subscription.current.queryOptions({ orgSlug }),
		),
	]);
	const membership = viewer.orgs.find((o) => o.slug === orgSlug);
	return {
		greeting: `Welcome back, ${viewer.name}`,
		orgName: org.name,
		role: membership?.role ?? org.role,
		canInvite: org.permissions.manageMembers,
		plan: `${subscription.plan} x${subscription.seats} (${subscription.interval})`,
		renewsInDays: Math.round(
			(new Date(subscription.renewsAt).getTime() - Date.now()) / 86_400_000,
		),
		projects: projects.pages.flatMap((page) => page.items).map(toProjectCard),
		hasMoreProjects: projects.pages.at(-1)?.nextCursor != null,
	};
}

export type Dashboard = Awaited<ReturnType<typeof loadDashboard>>;

export function watchProjects(
	input: ProjectListInput,
	onChange: (cards: ProjectCard[], hasMore: boolean) => void,
) {
	const observer = new InfiniteQueryObserver(queryClient, {
		...trpc.org.projects.list.infiniteQueryOptions(input, {
			getNextPageParam: (last) => last.nextCursor ?? undefined,
			placeholderData: keepPreviousData,
			select: (data) => ({
				...data,
				pages: data.pages.map((page) => ({
					...page,
					items: page.items.filter((p) => p.status !== "archived"),
				})),
			}),
		}),
	});
	const unsubscribe = observer.subscribe((result) => {
		if (result.status !== "success") return;
		onChange(
			result.data.pages.flatMap((page) => page.items.map(toProjectCard)),
			result.hasNextPage,
		);
	});
	return { observer, unsubscribe, loadMore: () => observer.fetchNextPage() };
}

export function boardKey(orgSlug: string, projectId: string) {
	return trpc.org.projects.tasks.board.queryKey({ orgSlug, projectId });
}

type TaskCommandInput = RouterInputs["org"]["projects"]["tasks"]["command"];

function applyOptimistic(board: Board, command: TaskCommandInput): Board {
	switch (command.type) {
		case "move": {
			const moving = [...board.todo, ...board.inProgress].find(
				(task) => task.id === command.taskId,
			);
			if (!moving) return board;
			const without = {
				...board,
				todo: board.todo.filter((task) => task.id !== command.taskId),
				inProgress: board.inProgress.filter(
					(task) => task.id !== command.taskId,
				),
			};
			if (command.to === "todo")
				return {
					...without,
					todo: [...without.todo, { ...moving, state: { kind: "todo" } }],
				};
			if (command.to === "in_progress") {
				const state = {
					kind: "in_progress" as const,
					startedAt: new Date().toISOString(),
				};
				return {
					...without,
					inProgress: [...without.inProgress, { ...moving, state }],
				};
			}
			return {
				...without,
				done: [...without.done, { id: moving.id, resolution: "fixed" }],
			};
		}
		case "assign":
			return {
				...board,
				todo: board.todo.map((task) =>
					task.id === command.taskId
						? { ...task, assigneeId: Id.nullable().parse(command.assigneeId) }
						: task,
				),
			};
		case "block":
		case "relabel":
			return board;
	}
}

export function createTaskCommander(orgSlug: string, projectId: string) {
	const key = boardKey(orgSlug, projectId);
	return new MutationObserver(
		queryClient,
		trpc.org.projects.tasks.command.mutationOptions({
			onMutate: async (input) => {
				await queryClient.cancelQueries({ queryKey: key });
				const previous = queryClient.getQueryData(key);
				if (previous)
					queryClient.setQueryData(key, applyOptimistic(previous, input));
				return { previous, startedAt: Date.now() };
			},
			onError: (error, _input, context) => {
				if (context?.previous) queryClient.setQueryData(key, context.previous);
				console.error(
					`Command failed after ${Date.now() - (context?.startedAt ?? 0)}ms`,
					fieldErrors(error),
				);
			},
			onSuccess: (result) => {
				if (result.type === "move")
					console.info(`${result.task.title}: ${result.from} -> ${result.to}`);
				queryClient.setQueryData(
					trpc.org.projects.tasks.byNumber.queryKey({
						orgSlug,
						projectId,
						number: result.task.number,
					}),
					result.task,
				);
			},
			onSettled: () =>
				Promise.all([
					queryClient.invalidateQueries({ queryKey: key }),
					queryClient.invalidateQueries(
						trpc.org.projects.tasks.list.infiniteQueryFilter({
							orgSlug,
							projectId,
						}),
					),
				]),
		}),
	);
}

export async function moveTask(
	orgSlug: string,
	projectId: string,
	taskId: string,
	to: "todo" | "in_progress" | "done",
) {
	const commander = createTaskCommander(orgSlug, projectId);
	const result = await commander.mutate({ orgSlug, type: "move", taskId, to });
	return result.type === "move" ? `${result.from}->${result.to}` : result.type;
}

export async function inviteMember(input: InviteMemberInput) {
	const mutation = new MutationObserver(
		queryClient,
		trpc.org.members.invite.mutationOptions({
			onSuccess: async (invitation) => {
				await queryClient.invalidateQueries(
					trpc.org.members.invitations.queryFilter({ orgSlug: input.orgSlug }),
				);
				queryClient.setQueryData(
					trpc.org.members.invitations.queryKey({ orgSlug: input.orgSlug }),
					(old) => (old ? [...old, invitation] : [invitation]),
				);
			},
		}),
	);
	try {
		const invitation = await mutation.mutate(input);
		return {
			ok: true as const,
			id: invitation.id,
			expiresAt: new Date(invitation.expiresAt),
		};
	} catch (error) {
		return { ok: false as const, errors: fieldErrors(error) };
	}
}

export async function changeRole(
	orgSlug: string,
	userId: string,
	role: Role,
	reason?: string,
) {
	const org = queryClient.getQueryData(trpc.org.bySlug.queryKey({ orgSlug }));
	if (org && roleRank[org.role] < roleRank.admin)
		throw new Error("Not allowed");
	const member = await client.org.members.updateRole.mutate({
		orgSlug,
		userId,
		role,
		reason,
	});
	queryClient.setQueryData(
		trpc.org.members.list.infiniteQueryKey({ orgSlug }),
		(data) =>
			data && {
				...data,
				pages: data.pages.map((page) => ({
					...page,
					items: page.items.map((m) =>
						m.userId === member.userId ? member : m,
					),
				})),
			},
	);
	return {
		name: member.user.name,
		role: member.role,
		since: new Date(member.joinedAt),
	};
}

export function invoiceQuery(orgSlug: string, invoiceId: string | null) {
	return trpc.billing.invoices.byId.queryOptions(
		invoiceId ? { orgSlug, id: invoiceId } : skipToken,
		{
			staleTime: Infinity,
			select: (invoice) => ({
				number: invoice.number,
				status: invoice.status,
				lines: invoice.lines.map((line) => ({
					label: line.description,
					amount: (line.quantity * line.unit.amountMinor) / 100,
					period: line.period
						? `${line.period.from} - ${line.period.to}`
						: null,
				})),
				total: `${(invoice.total.amountMinor / 100).toFixed(2)} ${invoice.total.currency}`,
				overdue:
					invoice.status === "open" && new Date(invoice.dueAt) < new Date(),
			}),
		},
	);
}

export function watchInvoice(
	orgSlug: string,
	invoiceId: string | null,
	render: (view: string) => void,
) {
	const observer = new QueryObserver(
		queryClient,
		invoiceQuery(orgSlug, invoiceId),
	);
	return observer.subscribe((result) => {
		if (result.isPending) return render("Loading...");
		if (result.isError)
			return render(
				`Error ${result.error.data?.code ?? "UNKNOWN"}: ${result.error.message}`,
			);
		render(
			`${result.data.number} ${result.data.total}${result.data.overdue ? " (overdue)" : ""}`,
		);
	});
}

export async function prefetchBilling(orgSlug: string) {
	await Promise.all([
		queryClient.prefetchInfiniteQuery({
			...trpc.billing.invoices.list.infiniteQueryOptions(
				{ orgSlug, status: "open" },
				{ getNextPageParam: (last) => last.nextCursor },
			),
			pages: 2,
		}),
		queryClient.prefetchQuery(
			trpc.billing.paymentMethods.list.queryOptions({ orgSlug }),
		),
		queryClient.prefetchQuery(
			trpc.billing.usage.queryOptions({
				orgSlug,
				metrics: ["api_calls", "seats"],
				range: { from: "2026-01-01", to: new Date() },
			}),
		),
	]);
	const methods =
		queryClient.getQueryData(
			trpc.billing.paymentMethods.list.queryKey({ orgSlug }),
		) ?? [];
	const primary = methods.find((m) => m.isDefault);
	const describe = (method: NonNullable<typeof primary>["method"]) => {
		switch (method.type) {
			case "card":
				return `${method.brand} ending ${method.last4} (${method.expMonth}/${method.expYear})`;
			case "sepa":
				return `SEPA ${method.iban}`;
			case "invoice":
				return `Invoice to ${method.billingEmail}, net ${method.netDays}`;
		}
	};
	return primary ? describe(primary.method) : "No payment method";
}

export async function usageSummary(
	orgSlug: string,
): Promise<Array<[string, number]>> {
	const usage: Usage = await queryClient.fetchQuery(
		trpc.billing.usage.queryOptions({
			orgSlug,
			metrics: ["api_calls", "storage_gb", "builds"],
			range: { from: new Date(Date.now() - 30 * 86_400_000), to: new Date() },
			granularity: "week",
		}),
	);
	return Object.entries(usage.totals).sort(([, a], [, b]) => b - a);
}

export function auditFeed(orgSlug: string) {
	const options = trpc.org.settings.audit.infiniteQueryOptions(
		{ orgSlug, limit: 50 },
		{ getNextPageParam: (last) => last.nextCursor, maxPages: 5 },
	);
	const describe = (row: AuditRow): string => {
		const who = row.actor.name;
		switch (row.event.action) {
			case "member.invited":
				return `${who} invited ${row.event.email} as ${row.event.role}`;
			case "member.role_changed":
				return `${who} changed ${row.event.userId} from ${row.event.from} to ${row.event.to}`;
			case "project.deleted":
				return `${who} deleted ${row.event.name}`;
			case "billing.plan_changed":
				return `${who} moved from ${row.event.from} to ${row.event.to} (${row.event.seats} seats)`;
			case "apikey.created":
				return `${who} created a key with ${row.event.scopes.join(", ")}`;
		}
	};
	return {
		options,
		async load() {
			const data: InfiniteData<
				RouterOutputs["org"]["settings"]["audit"],
				string | null
			> = await queryClient.fetchInfiniteQuery(options);
			return data.pages.flatMap((page) =>
				page.items.map((row) => ({
					at: new Date(row.at),
					text: describe(row),
				})),
			);
		},
	};
}

export function notificationsSubscription(onUnread: (count: number) => void) {
	let unread = 0;
	const options = trpc.viewer.onNotification.subscriptionOptions(undefined, {
		onData: (event) => {
			if (!event.data.read) unread += 1;
			if (
				event.data.payload.kind === "billing" &&
				event.data.payload.severity === "critical"
			) {
				void queryClient.invalidateQueries(trpc.billing.pathFilter());
			}
			onUnread(unread);
		},
		onConnectionStateChange: (state) => {
			if (state.state === "connecting" && state.error)
				console.warn(state.error.message);
		},
	});
	return options.subscribe({});
}

export async function createProjectFlow(
	orgSlug: string,
	name: string,
	key: string,
) {
	const created = await client.org.projects.create.mutate({
		orgSlug,
		name,
		key,
		description: null,
		visibility: "private",
		tags: [],
		template: "kanban",
	});
	queryClient.setQueryData(
		trpc.org.projects.byId.queryKey({ orgSlug, id: created.id }),
		created,
	);
	await queryClient.invalidateQueries(trpc.org.projects.pathFilter());
	const [firstTask] =
		(
			await queryClient.fetchInfiniteQuery(
				trpc.org.projects.tasks.list.infiniteQueryOptions(
					{ orgSlug, projectId: created.id, assignee: "me" },
					{ getNextPageParam: (last) => last.nextCursor },
				),
			)
		).pages[0]?.items ?? [];
	const state = firstTask?.state;
	return {
		card: toProjectCard(created),
		firstTask:
			state?.kind === "blocked"
				? `blocked by ${state.blockedBy.length}`
				: (state?.kind ?? "none"),
		cacheKey: hashKey(
			trpc.org.projects.byId.queryKey({ orgSlug, id: created.id }),
		),
	};
}

export async function rotateWebhook(orgSlug: string, id: string) {
	const hooks = await queryClient.fetchQuery(
		trpc.org.settings.webhooks.list.queryOptions({ orgSlug }),
	);
	const hook = hooks.find((h) => h.id === id);
	if (!hook) return null;
	const updated = await client.org.settings.webhooks.upsert.mutate({
		orgSlug,
		...hook,
		rotateSecret: true,
	});
	const test = await client.org.settings.webhooks.test.mutate({
		orgSlug,
		id: updated.id,
		event: updated.events[0] ?? "project.created",
	});
	return {
		secret: updated.secret,
		delivered: test.delivered && test.status < 300,
	};
}

export async function lookupMany(ids: string[]) {
	const users = await Promise.all(
		ids.map((id) =>
			queryClient.fetchQuery(trpc.lookupUser.queryOptions({ id })),
		),
	);
	return users.flatMap((user) =>
		user
			? [{ id: user.id, label: user.name, avatar: user.avatarUrl ?? undefined }]
			: [],
	);
}

export async function health() {
	const [status, sessions] = await Promise.all([
		client.health.query(),
		client.viewer.sessions.query(),
	]);
	return {
		status: status.status,
		since: status.time,
		activeSessions: sessions.filter((s) => !s.current).length,
	};
}
