/* should not generate diagnostics */
import { QueryClient, queryOptions, skipToken, useInfiniteQuery, useQuery, useSuspenseQuery } from "@tanstack/react-query";

declare const enabled: boolean;
declare function untyped(): any;

useQuery({
	queryKey: ["test"],
	queryFn: () => ({ data: "test" }),
});

useQuery({
	queryKey: ["test"],
	queryFn: async () => ({ data: "test" }),
});

useQuery({
	queryKey: ["test"],
	queryFn: () => Promise.resolve({ data: "test" }),
});

interface Data {
	value: string;
}

useQuery({
	queryKey: ["test"],
	queryFn: async (): Promise<Data> => {
		return { value: "test" };
	},
});

interface Response<T> {
	data: T;
}

useQuery({
	queryKey: ["test"],
	queryFn: async (): Promise<Response<string>> => {
		return { data: "test" };
	},
});

async function fetchData(): Promise<{ data: string }> {
	return { data: "test" };
}

useQuery({
	queryKey: ["test"],
	queryFn: fetchData,
});

// `null` and other falsy values are valid query data.
useQuery({
	queryKey: ["test"],
	queryFn: () => null,
});

useQuery({
	queryKey: ["test"],
	queryFn: () => 0,
});

useQuery({
	queryKey: ["test"],
	queryFn: () => false,
});

useQuery({
	queryKey: ["test"],
	queryFn: async (): Promise<string | null> => null,
});

useInfiniteQuery({
	queryKey: ["test"],
	queryFn: ({ pageParam }) => ({ data: "test", page: pageParam }),
	initialPageParam: 0,
	// Only `queryFn` is checked.
	getNextPageParam: (lastPage) => undefined,
});

useSuspenseQuery({
	queryKey: ["test"],
	queryFn: () => ({ data: "test" }),
});

const options = queryOptions({
	queryKey: ["test"],
	queryFn: () => ({ data: "test" }),
});

const queryClient = new QueryClient();

queryClient.fetchQuery({
	queryKey: ["test"],
	queryFn: () => fetch("/api/test").then((r) => r.json()),
});

queryClient.prefetchInfiniteQuery({
	queryKey: ["test"],
	queryFn: ({ pageParam }: { pageParam: number }) =>
		fetch(`/api/test?page=${pageParam}`).then((r) => r.json()),
	initialPageParam: 0,
});

enum ExampleEnum {
	A,
	B,
}

useQuery({
	queryKey: ["test"],
	queryFn: () => ExampleEnum.A,
});

enum StringEnum {
	Foo = "foo",
	Bar = "bar",
}

useQuery({
	queryKey: ["test"],
	queryFn: () => StringEnum.Foo,
});

useQuery({
	queryKey: ["test"],
	queryFn: async () => {
		return ExampleEnum.B;
	},
});

const enum Direction {
	Up = "UP",
	Down = "DOWN",
}

useQuery({
	queryKey: ["test"],
	queryFn: () => Direction.Up,
});

// Every path returns or throws.
useQuery({
	queryKey: ["test"],
	queryFn: async () => {
		const response = await fetch("/api/test");
		if (!response.ok) {
			throw new Error("Request failed");
		}
		return "loaded";
	},
});

useQuery({
	queryKey: ["test"],
	queryFn: async () => {
		try {
			return { data: "test" };
		} catch (error) {
			return null;
		}
	},
});

useQuery({
	queryKey: ["test"],
	queryFn: (context) => {
		switch (context.queryKey[0]) {
			case "test":
				return 1;
			default:
				return 0;
		}
	},
});

useQuery({
	queryKey: ["test"],
	queryFn: async () => {
		try {
			return await fetchData();
		} finally {
			console.log("done");
		}
	},
});

// The cases cover every value, so the end of the function is unreachable.
declare const kind: "a" | "b";
declare function fail(message: string): never;

useQuery({
	queryKey: ["test"],
	queryFn: () => {
		switch (kind) {
			case "a":
				return 1;
			case "b":
				return 2;
		}
	},
});

useQuery({
	queryKey: ["test"],
	queryFn() {
		switch (kind) {
			case "a":
				return 1;
			default:
				fail("Unknown kind");
		}
	},
});

// A `switch` statement that doesn't cover every value isn't detected.
useQuery({
	queryKey: ["test"],
	queryFn: (context) => {
		switch (context.queryKey[0]) {
			case "test":
				return 1;
		}
	},
});

useQuery({
	queryKey: ["test"],
	queryFn: (context) => {
		switch (context.queryKey[0]) {
			case "test":
				console.log("test");
		}
	},
});

// The function called at the end never returns.
useQuery({
	queryKey: ["test"],
	queryFn: () => {
		fail("Not implemented");
	},
});

useQuery({
	queryKey: ["test"],
	queryFn: () => {
		switch (kind) {
			case "a":
				throw new Error("Not implemented");
			case "b":
				throw new Error("Not implemented");
		}
	},
});

useQuery({
	queryKey: ["test"],
	queryFn: async () => {
		const response = await fetch("/api/test");
		if (response.ok) {
			return "loaded";
		}
		fail("Request failed");
	},
});

// A function that always throws never gives the query any data.
useQuery({
	queryKey: ["test"],
	queryFn: () => {
		throw new Error("Not implemented");
	},
});

useQuery({
	queryKey: ["test"],
	queryFn: async () => {
		throw new Error("Not implemented");
	},
});

useQuery({
	queryKey: ["test"],
	queryFn() {
		throw new Error("Not implemented");
	},
});

useQuery({
	queryKey: ["test"],
	queryFn: () => Promise.reject(new Error("Not implemented")),
});

// Functions and methods nested in the query function have their own `return` statements.
useQuery({
	queryKey: ["test"],
	queryFn: async () => {
		const log = () => {
			console.log("fetching");
		};
		log();
		return "test";
	},
});

useQuery({
	queryKey: ["test"],
	queryFn: async () => {
		const visitor = {
			enter() {
				return;
			},
		};
		visitor.enter();
		return "test";
	},
});

// Function expressions, methods, and shorthand properties that return data.
useQuery({
	queryKey: ["test"],
	queryFn: async function () {
		return "test";
	},
});

useQuery({
	queryKey: ["test"],
	async queryFn() {
		return "test";
	},
});

const queryFn = async () => "test";

useQuery({
	queryKey: ["test"],
	queryFn,
});

// The return type is unknown, so nothing is reported.
useQuery({
	queryKey: ["test"],
	queryFn: () => untyped(),
});

useQuery({
	queryKey: ["test"],
	queryFn: notDeclaredAnywhere,
});

// `skipToken` isn't a function.
useQuery({
	queryKey: ["test"],
	queryFn: enabled ? fetchData : skipToken,
});

// Other properties can hold functions that return nothing.
useQuery({
	queryKey: ["test"],
	queryFn: fetchData,
	select: () => {},
	[queryFn.name]: () => {},
});

// A computed name isn't checked.
const key = "queryFn";
const computed = {
	[key]: () => {},
};

// A check before the `return` statement can rule out `undefined`.
declare const cached: Data | undefined;
declare const cache: { entry: Data | undefined };

useQuery({
	queryKey: ["test"],
	queryFn: () => {
		if (!cached) {
			throw new Error("Missing");
		}
		return cached;
	},
});

useQuery({
	queryKey: ["test"],
	queryFn: () => {
		if (cached === undefined) {
			return null;
		}
		return cached;
	},
});

useQuery({
	queryKey: ["test"],
	queryFn: async () => {
		if (!cache.entry) {
			throw new Error("Missing");
		}
		return cache.entry;
	},
});

useQuery({
	queryKey: ["test"],
	queryFn: () => (cached ? cached : null),
});

useQuery({
	queryKey: ["test"],
	queryFn: () => cached ?? null,
});

// A function that is defined somewhere else is only reported when it never returns data.
async function findData(): Promise<Data | undefined> {
	return cached;
}

useQuery({
	queryKey: ["test"],
	queryFn: findData,
});

function readCache() {
	if (!cached) {
		throw new Error("Missing");
	}
	return cached;
}

useQuery({
	queryKey: ["test"],
	queryFn: readCache,
});

// Getters and classes nested in the query function have their own `return` statements.
useQuery({
	queryKey: ["test"],
	queryFn: async () => {
		const source = {
			get value() {
				return;
			},
		};
		class Reader {
			read() {
				return;
			}
		}
		new Reader().read();
		return String(source.value);
	},
});

// Generator functions aren't checked.
useQuery({
	queryKey: ["test"],
	queryFn: function* () {},
});

useQuery({
	queryKey: ["test"],
	*queryFn() {},
});
