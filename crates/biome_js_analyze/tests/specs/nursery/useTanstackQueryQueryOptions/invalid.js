/* should generate diagnostics */
import {
	QueryClient,
	QueryClient as Client,
	useInfiniteQuery,
	useIsFetching,
	usePrefetchInfiniteQuery,
	usePrefetchQuery,
	useQueries,
	useQuery,
	useQuery as useData,
	useQueryClient,
	useQueryClient as getClient,
	useSuspenseInfiniteQuery,
	useSuspenseQueries,
	useSuspenseQuery,
} from "@tanstack/react-query";
import { useQuery as useVueQuery } from "@tanstack/vue-query";

// Options written in a hook.
function Hooks({ id, queryKey, queryFn }) {
	useQuery({
		queryKey: ["todo", id],
		queryFn: () => fetchTodo(id),
	});
	useQuery({ queryKey: ["todos"] });
	useQuery({ queryFn: () => fetchTodos() });
	useQuery({ queryKey, queryFn });
	useQuery({
		queryKey: ["todos"],
		queryFn() {
			return fetchTodos();
		},
	});
	useQuery({ "queryKey": ["todos"], "queryFn": fetchTodos });
	useQuery({ ...options, queryKey: ["override"] });
	useQuery({ ...options, queryFn: () => fetchOverride() });
	useQuery(({ queryKey: ["todos"], queryFn: fetchTodos }));
	useData({ queryKey: ["todos"], queryFn: fetchTodos });
	useVueQuery({ queryKey: ["todos"], queryFn: fetchTodos });
	useSuspenseQuery({ queryKey: ["todos"], queryFn: fetchTodos });
	usePrefetchQuery({ queryKey: ["todos"], queryFn: fetchTodos });
	useInfiniteQuery({
		queryKey: ["todos"],
		queryFn: ({ pageParam }) => fetchTodos(pageParam),
		initialPageParam: 0,
		getNextPageParam: (lastPage) => lastPage.nextCursor,
	});
	useSuspenseInfiniteQuery({ queryKey: ["todos"], queryFn: fetchTodos });
	usePrefetchInfiniteQuery({ queryKey: ["todos"], queryFn: fetchTodos });
}

// Options written in a list of queries.
function Lists({ ids, isEnabled }) {
	useQueries({
		queries: [
			{ queryKey: ["todos"], queryFn: fetchTodos },
			userOptions,
			{ queryKey: ["users"], queryFn: fetchUsers },
		],
	});
	useSuspenseQueries({
		queries: [{ queryKey: ["todos"], queryFn: fetchTodos }],
	});
	useQueries({
		queries: ids.map((id) => ({
			queryKey: ["todo", id],
			queryFn: () => fetchTodo(id),
		})),
	});
	useSuspenseQueries({
		queries: ids.map(function (id) {
			return { queryKey: ["todo", id], queryFn: () => fetchTodo(id) };
		}),
	});
	useQueries({
		queries: ids.map((id) => {
			if (!id) {
				return { queryKey: ["todo", "none"], queryFn: () => null };
			}
			return isEnabled
				? { queryKey: ["todo", id], queryFn: () => fetchTodo(id) }
				: disabledOptions;
		}),
	});
	useQueries({
		queries: ids.map(
			(id) => cachedOptions[id] ?? { queryKey: ["todo", id], queryFn: fetchTodo },
		),
	});
}

// Options written in a method of the query client.
function ClientOptions() {
	const queryClient = useQueryClient();
	queryClient.fetchQuery({ queryKey: ["todos"], queryFn: fetchTodos });
	queryClient.prefetchQuery({ queryKey: ["todos"], queryFn: fetchTodos });
	queryClient.ensureQueryData({ queryKey: ["todos"], queryFn: fetchTodos });
	queryClient.fetchInfiniteQuery({ queryKey: ["todos"], queryFn: fetchTodos });
	queryClient.prefetchInfiniteQuery({ queryKey: ["todos"], queryFn: fetchTodos });
	queryClient.ensureInfiniteQueryData({ queryKey: ["todos"], queryFn: fetchTodos });
}

// A query key written in a method of the query client.
function ClientKeys() {
	const queryClient = useQueryClient();
	queryClient.getQueryData(["todos"]);
	queryClient.setQueryData(["todos"], []);
	queryClient.getQueryState(["todos"]);
	queryClient.setQueryDefaults(["todos"], { staleTime: 1000 });
	queryClient.getQueryDefaults(["todos"]);
}

// A query key written in filters.
function Filters() {
	const queryClient = useQueryClient();
	queryClient.invalidateQueries({ queryKey: ["todos"] });
	queryClient.invalidateQueries({ queryKey: ["todos"], exact: true });
	queryClient.cancelQueries({ queryKey: ["todos"] });
	queryClient.refetchQueries({ queryKey: ["todos"] });
	queryClient.removeQueries({ queryKey: ["todos"] });
	queryClient.resetQueries({ queryKey: ["todos"] });
	queryClient.isFetching({ queryKey: ["todos"] });
	queryClient.getQueriesData({ queryKey: ["todos"] });
	queryClient.setQueriesData({ queryKey: ["todos"] }, []);
	useIsFetching({ queryKey: ["todos"] });
}

// The ways to get a query client.
function Clients() {
	const client = getClient();
	client.fetchQuery({ queryKey: ["todos"], queryFn: fetchTodos });

	const alias = client;
	alias.getQueryData(["todos"]);
	alias?.invalidateQueries({ queryKey: ["todos"] });
	alias["getQueryState"](["todos"]);

	useQueryClient().getQueryData(["todos"]);
	new QueryClient().getQueryData(["todos"]);
}

const sharedClient = new Client();
sharedClient.fetchQuery({ queryKey: ["todos"], queryFn: fetchTodos });

function usesLaterClient() {
	laterClient.getQueryData(["todos"]);
}
const laterClient = new QueryClient();
