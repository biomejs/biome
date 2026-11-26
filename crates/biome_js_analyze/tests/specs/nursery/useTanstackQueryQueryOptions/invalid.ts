/* should generate diagnostics */
import {
	type QueryClient as QueryClientType,
	QueryClient,
	useQuery,
	useQueryClient,
	type UseQueryOptions,
} from "@tanstack/react-query";

function Component() {
	const queryClient = useQueryClient();
	queryClient.getQueryData(["todos"] as const);
	queryClient.getQueryData((["todos"]) satisfies readonly string[]);
	queryClient.getQueryData(<const>["todos"]);
	queryClient.invalidateQueries({ queryKey: ["todos"] as const });
	queryClient.invalidateQueries({
		queryKey: (["todos"]) satisfies readonly string[],
	});

	useQuery({ queryKey: ["todos"], queryFn: fetchTodos } satisfies UseQueryOptions);
	useQuery<Todo[]>({ queryKey: ["todos"], queryFn: fetchTodos });
}

const typedClient: QueryClientType = new QueryClient();
typedClient.fetchQuery({ queryKey: ["todos"], queryFn: fetchTodos });
(typedClient as QueryClientType)!.getQueryData(["todos"]);
