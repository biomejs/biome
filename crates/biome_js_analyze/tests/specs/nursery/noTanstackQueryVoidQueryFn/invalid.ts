/* should generate diagnostics */
import { QueryClient, queryOptions, useInfiniteQuery, useQuery, useSuspenseQuery } from "@tanstack/react-query";

declare function someOperation(): Promise<number>;

// The function body never returns.
useQuery({
	queryKey: ["test"],
	queryFn: () => {
		console.log("test");
	},
});

// The function returns `undefined`.
useQuery({
	queryKey: ["test"],
	queryFn: () => undefined,
});

// The async function body never returns.
useQuery({
	queryKey: ["test"],
	queryFn: async () => {
		await someOperation();
	},
});

// The return type says that the promise has no result.
useQuery({
	queryKey: ["test"],
	queryFn: async (): Promise<void> => {
		await someOperation();
	},
});

// The return type is `void`.
useQuery({
	queryKey: ["test"],
	queryFn: (): void => {
		console.log("test");
	},
});

// The returned promise results in `undefined`.
useQuery({
	queryKey: ["test"],
	queryFn: () => Promise.resolve(undefined),
});

// The function is declared somewhere else.
async function voidOperation(): Promise<void> {
	await someOperation();
}

useQuery({
	queryKey: ["test"],
	queryFn: voidOperation,
});

// Only one path returns.
useQuery({
	queryKey: ["test"],
	queryFn: () => {
		if (Math.random() > 0.5) {
			return { data: "test" };
		}
	},
});

// One branch of the conditional expression is `undefined`.
useQuery({
	queryKey: ["test"],
	queryFn: () => (Math.random() > 0.5 ? { data: "test" } : undefined),
});

// The `catch` clause doesn't return.
useQuery({
	queryKey: ["test"],
	queryFn: async () => {
		try {
			return { data: "test" };
		} catch (error) {
			console.error(error);
		}
	},
});

// The declared return type includes `undefined`.
useQuery({
	queryKey: ["test"],
	queryFn: async (): Promise<string | undefined> => {
		return "test";
	},
});

useInfiniteQuery({
	queryKey: ["test"],
	queryFn: async ({ pageParam }) => {
		await fetch("/api/test?page=" + pageParam);
	},
	initialPageParam: 0,
	getNextPageParam: (lastPage) => undefined,
});

useSuspenseQuery({
	queryKey: ["test"],
	queryFn: () => {
		console.log("fetching");
	},
});

const options = queryOptions({
	queryKey: ["test"],
	queryFn: async () => {
		await fetch("/api/test");
	},
});

const queryClient = new QueryClient();

queryClient.fetchQuery({
	queryKey: ["test"],
	queryFn: async () => {
		await fetch("/api/test");
	},
});

queryClient.prefetchInfiniteQuery({
	queryKey: ["test"],
	queryFn: async ({ pageParam }: { pageParam: number }) => {
		await fetch(`/api/test?page=${pageParam}`);
	},
	initialPageParam: 0,
});

// The returned call can result in `undefined`.
declare function findData(): { data: string } | undefined;

useQuery({
	queryKey: ["test"],
	queryFn: () => findData(),
});

useQuery({
	queryKey: ["test"],
	queryFn: async () => {
		return findData();
	},
});

// The returned variable is always `undefined`.
useQuery({
	queryKey: ["test"],
	queryFn: async () => {
		const result = await voidOperation();
		return result;
	},
});

// A `return` statement without a value returns `undefined`.
useQuery({
	queryKey: ["test"],
	queryFn: async () => {
		const response = await fetch("/api/test");
		if (!response.ok) {
			return;
		}
		return response.json();
	},
});

// The value of the other path is unknown, but the end of the function is still reachable.
useQuery({
	queryKey: ["test"],
	queryFn: async () => {
		const response = await fetch("/api/test");
		if (response.ok) {
			return response.json();
		}
	},
});

// An async function returns the result of a function that returns a promise without a result.
useQuery({
	queryKey: ["test"],
	queryFn: async () => voidOperation(),
});

useQuery({
	queryKey: ["test"],
	queryFn: async () => {
		return voidOperation();
	},
});

// The call at the end returns normally, so the end of the function is reachable.
useQuery({
	queryKey: ["test"],
	queryFn: async () => {
		if (Math.random() > 0.5) {
			return { data: "test" };
		}
		await someOperation();
	},
});

// Leaving a loop reaches the end of the function.
useQuery({
	queryKey: ["test"],
	queryFn: async () => {
		for (const url of ["/api/a", "/api/b"]) {
			const response = await fetch(url);
			if (response.ok) {
				return url;
			}
		}
	},
});

useQuery({
	queryKey: ["test"],
	queryFn: async () => {
		try {
			await someOperation();
		} finally {
			console.log("done");
		}
	},
});

// Function expressions, methods, and shorthand properties are checked too.
useQuery({
	queryKey: ["test"],
	queryFn: async function () {
		await someOperation();
	},
});

useQuery({
	queryKey: ["test"],
	async queryFn() {
		await someOperation();
	},
});

useQuery({
	queryKey: ["test"],
	queryFn() {
		if (Math.random() > 0.5) {
			return { data: "test" };
		}
	},
});

useQuery({
	queryKey: ["test"],
	queryFn() {
		return undefined;
	},
});

useQuery({
	queryKey: ["test"],
	async queryFn(): Promise<void> {
		await someOperation();
	},
});

const queryFn = async () => {
	await someOperation();
};

useQuery({
	queryKey: ["test"],
	queryFn,
});

// The name can be quoted, and the function can be in parentheses.
useQuery({
	queryKey: ["test"],
	"queryFn": (async () => {
		await someOperation();
	}),
});

// Query options don't have to be passed to a TanStack Query function directly.
const sharedOptions = {
	queryKey: ["test"],
	queryFn: voidOperation,
};
