import {
	derived,
	fromStore,
	get,
	readable,
	readonly,
	toStore,
	writable,
	type Readable,
} from "./vendor/svelte-store";
import * as v from "./vendor/valibot";
import {
	normalizedSchema,
	draftSchema,
	type WorkspaceInput,
	type Workspace,
	type Task,
} from "./schema";

export function createWorkspaceStores(initial: WorkspaceInput) {
	const draft = writable(initial);
	const selectedProject = writable<string | null>(null);
	const filters = writable({
		search: "",
		kinds: ["onsite", "remote", "delivery"] as Task["kind"][],
		assignees: [] as string[],
		minimumBudget: 0,
	});
	const exchangeRates = readable({ USD: 1, EUR: 0.92, GBP: 0.79 });
	const pagination = writable({
		page: 0,
		size: 25,
		sort: "title" as "title" | "assignees",
	});
	const pending = writable<
		Record<
			string,
			{ revision: number; status: "saving" | "failed"; message?: string }
		>
	>({});

	const validation = derived(draft, ($draft) =>
		v.safeParse(normalizedSchema, $draft),
	);
	const workspace = derived(validation, ($validation) =>
		$validation.success ? $validation.output : null,
	);
	const errors = derived(validation, ($validation) =>
		$validation.success ? null : v.flatten($validation.issues),
	);
	const projects = derived(
		[workspace, filters, exchangeRates],
		([$workspace, $filters, $rates]) =>
			($workspace?.projects ?? [])
				.filter(
					(project) =>
						project.name
							.toLowerCase()
							.includes($filters.search.toLowerCase()) &&
						project.budget >= $filters.minimumBudget,
				)
				.map((project) => ({
					...project,
					convertedBudget:
						project.budget * $rates[$workspace?.preferences.currency ?? "USD"],
				})),
	);
	const activeProject = derived(
		[projects, selectedProject],
		([$projects, $selected]) =>
			$projects.find((project) => project.id === $selected) ??
			$projects[0] ??
			null,
	);
	const tasks = derived([activeProject, filters], ([$project, $filters]) =>
		($project?.tasks ?? []).filter(
			(task) =>
				$filters.kinds.includes(task.kind) &&
				($filters.assignees.length === 0 ||
					task.assignees.some((member) =>
						$filters.assignees.includes(member.id),
					)),
		),
	);
	const grouped = derived(tasks, ($tasks) => ({
		onsite: $tasks
			.filter((task) => task.kind === "onsite")
			.map((task) => ({
				...task,
				equipmentCost: task.equipment.reduce(
					(sum, item) => sum + item.cost.amount * item.count,
					0,
				),
			})),
		remote: $tasks
			.filter((task) => task.kind === "remote")
			.map((task) => ({
				...task,
				duration: task.recordings.reduce(
					(sum, recording) => sum + recording.duration,
					0,
				),
			})),
		delivery: $tasks
			.filter((task) => task.kind === "delivery")
			.map((task) => ({
				...task,
				weight: task.packages.reduce((sum, item) => sum + item.weight, 0),
			})),
	}));
	const page = derived([tasks, pagination], ([$tasks, $pagination]) => ({
		total: $tasks.length,
		rows: [...$tasks]
			.sort((left, right) =>
				$pagination.sort === "title"
					? left.title.localeCompare(right.title)
					: left.assignees.length - right.assignees.length,
			)
			.slice(
				$pagination.page * $pagination.size,
				($pagination.page + 1) * $pagination.size,
			),
	}));
	const summary = derived(
		[workspace, projects, grouped, errors, pending],
		([$workspace, $projects, $grouped, $errors, $pending]) => ({
			organization: $workspace?.organization.name ?? "",
			budget: $projects.reduce(
				(sum, project) => sum + project.convertedBudget,
				0,
			),
			counts: {
				onsite: $grouped.onsite.length,
				remote: $grouped.remote.length,
				delivery: $grouped.delivery.length,
			},
			equipment: $grouped.onsite.flatMap((task) => task.equipment),
			failed: Object.entries($pending).filter(
				([, request]) => request.status === "failed",
			),
			canSubmit: $errors === null && Object.keys($pending).length === 0,
		}),
	);
	const delayedSummary = derived(
		summary,
		($summary, set) => {
			const timer = setTimeout(
				() => set({ ...$summary, generatedAt: Date.now() }),
				25,
			);
			return () => clearTimeout(timer);
		},
		{ ...get(summary), generatedAt: 0 },
	);
	const dashboard = derived(
		[page, summary, delayedSummary, activeProject],
		([$page, $summary, $delayed, $active]) => ({
			...$summary,
			page: $page,
			delayed: $delayed,
			owner: $active?.owner ?? null,
			taskLinks: $page.rows.map((task) => ({
				id: task.id,
				href: `/projects/${$active?.id}/tasks/${task.id}`,
				kind: task.kind,
			})),
		}),
	);
	const current = fromStore(draft);
	const organizationName = toStore(
		() => current.current.organization.name,
		(name) => {
			draft.update((previous) => ({
				...previous,
				organization: { ...previous.organization, name },
			}));
		},
	);
	function patchProject(
		id: string,
		patch: Partial<WorkspaceInput["projects"][number]>,
	) {
		draft.update((previous) => ({
			...previous,
			projects: previous.projects.map((project) =>
				project.id === id ? { ...project, ...patch } : project,
			),
		}));
	}
	function reset(input: unknown) {
		const result = v.safeParse(draftSchema, input);
		if (result.success)
			draft.update((previous) => ({ ...previous, ...result.output }));
	}
	return {
		draft,
		filters,
		selectedProject,
		pagination,
		pending,
		workspace,
		errors,
		grouped,
		dashboard,
		organizationName,
		summary: readonly(summary),
		patchProject,
		reset,
	};
}

export type WorkspaceStores = ReturnType<typeof createWorkspaceStores>;
export type StoreValue<T> = T extends Readable<infer Value> ? Value : never;
export type StoreValues = {
	[Key in keyof WorkspaceStores]: StoreValue<WorkspaceStores[Key]>;
};
export type Dashboard = StoreValues["dashboard"];
export type TaskRows = Dashboard["page"]["rows"][number];
export type TaskViews = {
	[Kind in TaskRows["kind"]]: Extract<TaskRows, { kind: Kind }>;
};
export declare const initialWorkspace: WorkspaceInput;
export const stores = createWorkspaceStores(initialWorkspace);
export const dashboard = get(stores.dashboard);
export const ownerEmail = dashboard.owner?.email;
export const onsiteEquipment = get(stores.grouped).onsite.flatMap(
	(task) => task.equipment,
);
export const organization = fromStore(stores.organizationName).current;
export const parsedWorkspace: Workspace = v.parse(
	normalizedSchema,
	initialWorkspace,
);
