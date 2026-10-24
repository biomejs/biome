/* should not generate diagnostics */
import { queryOptions, useQuery } from "@tanstack/react-query";

useQuery({
	queryKey: ["todos"],
	queryFn: async () => {
		const response = await fetch("/api/todos");
		return response.json();
	},
});

useQuery({
	queryKey: ["todos"],
	queryFn: () => fetch("/api/todos").then((response) => response.json()),
});

useQuery({
	queryKey: ["todos"],
	queryFn: async () => {
		const response = await fetch("/api/todos");
		if (!response.ok) {
			return null;
		}
		return response.json();
	},
});

async function fetchTodos() {
	const response = await fetch("/api/todos");
	return response.json();
}

export const todosOptions = queryOptions({
	queryKey: ["todos"],
	queryFn: fetchTodos,
});

const queryFn = () => [];

export const sharedOptions = { queryKey: ["todos"], queryFn };

// The function comes from a parameter, so its return type is unknown.
export function createOptions(fetcher) {
	return { queryKey: ["todos"], queryFn: fetcher };
}

// An object without a `queryFn` property isn't checked.
export const handlers = {
	onSuccess: () => {},
};
