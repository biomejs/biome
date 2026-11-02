/* should generate diagnostics */
import { queryOptions, useQuery } from "@tanstack/react-query";

useQuery({
	queryKey: ["todos"],
	queryFn: async () => {
		await fetch("/api/todos");
	},
});

useQuery({
	queryKey: ["todos"],
	queryFn: async () => {
		const response = await fetch("/api/todos");
		if (response.ok) {
			return response.json();
		}
	},
});

useQuery({
	queryKey: ["todos"],
	queryFn: function () {
		fetch("/api/todos").then((response) => response.json());
	},
});

async function refreshTodos() {
	await fetch("/api/todos/refresh");
}

export const todosOptions = queryOptions({
	queryKey: ["todos"],
	queryFn: refreshTodos,
});

const queryFn = () => {};

export const sharedOptions = { queryKey: ["todos"], queryFn };
