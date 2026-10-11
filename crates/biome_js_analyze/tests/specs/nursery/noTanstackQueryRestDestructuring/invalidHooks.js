/* should generate diagnostics */
import {
	useQuery,
	useInfiniteQuery,
	useSuspenseQuery,
	useSuspenseInfiniteQuery,
	useQueries,
	useSuspenseQueries,
} from "@tanstack/react-query";

function Component() {
	const { data, ...rest } = useQuery();
	const { data: infiniteData, ...infiniteRest } = useInfiniteQuery();
	const { data: suspenseData, ...suspenseRest } = useSuspenseQuery();
	const { data: suspenseInfiniteData, ...suspenseInfiniteRest } = useSuspenseInfiniteQuery();
	const [query1, { data: queriesData, ...queriesRest }] = useQueries([
		{ queryKey: ["key1"], queryFn: () => {} },
		{ queryKey: ["key2"], queryFn: () => {} },
	]);
	const [suspenseQuery1, { data: suspenseQueriesData, ...suspenseQueriesRest }] = useSuspenseQueries([
		{ queryKey: ["key1"], queryFn: () => {} },
		{ queryKey: ["key2"], queryFn: () => {} },
	]);
	return;
}

function SpreadInReturn() {
	const query = useQuery();
	return { ...query, data: query.data[0] };
}

function SpreadInObjectExpression() {
	const query = useQuery();
	const result = { ...query, data: query.data[0] };
	return result;
}

function StoredQueries() {
	const queries = useQueries([]);
	const suspenseQueries = useSuspenseQueries([]);
	const { ...queriesRest } = queries;
	return [...suspenseQueries, queriesRest];
}
