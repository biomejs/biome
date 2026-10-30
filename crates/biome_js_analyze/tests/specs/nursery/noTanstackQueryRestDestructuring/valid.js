/* should not generate diagnostics */
import {
	useInfiniteQuery,
	useMutation,
	useQueries,
	useQuery,
	useSuspenseInfiniteQuery,
	useSuspenseQueries,
	useSuspenseQuery,
} from "@tanstack/react-query";

function NotCaptured() {
	useQuery();
	useInfiniteQuery();
	useSuspenseQuery();
	useSuspenseInfiniteQuery();
	useQueries([]);
	useSuspenseQueries([]);
}

function NotDestructured() {
	const query = useQuery();
	const infiniteQuery = useInfiniteQuery();
	const suspenseQuery = useSuspenseQuery();
	const suspenseInfiniteQuery = useSuspenseInfiniteQuery();
	const queries = useQueries([]);
	const suspenseQueries = useSuspenseQueries([]);
	return [query, infiniteQuery, suspenseQuery, suspenseInfiniteQuery, queries, suspenseQueries];
}

function DestructuredWithoutRest() {
	const { data, isLoading, isError } = useQuery();
	const { data: pages } = useInfiniteQuery();
	const { data: suspended } = useSuspenseQuery();
	const { data: suspendedPages } = useSuspenseInfiniteQuery();
	return [data, isLoading, isError, pages, suspended, suspendedPages];
}

function QueriesWithoutObjectRest() {
	const [query1, { data, isLoading }, , ...others] = useQueries([
		{ queryKey: ["key1"], queryFn: () => {} },
		{ queryKey: ["key2"], queryFn: () => {} },
		{ queryKey: ["key3"], queryFn: () => {} },
		{ queryKey: ["key4"], queryFn: () => {} },
		{ queryKey: ["key5"], queryFn: () => {} },
	]);
	const [query2, { data: suspended }] = useSuspenseQueries([
		{ queryKey: ["key1"], queryFn: () => {} },
		{ queryKey: ["key2"], queryFn: () => {} },
	]);
	return [query1, data, isLoading, others, query2, suspended];
}

function QueryListCopies() {
	const queries = useQueries([]);
	const copy = [...queries];
	const { length, ...indexed } = queries;
	return [copy, length, indexed, Math.max(...queries)];
}

function QueryResultSpreadOutsideObject() {
	const query = useQuery();
	return [[...query], Math.max(...query), [...useQuery()]];
}

function NestedRest() {
	const {
		data: { first, ...otherData },
	} = useQuery();
	const query = useQuery();
	const {
		data: { second, ...moreData },
	} = query;
	return [first, otherData, second, moreData];
}

function PropertyOfQueryResult() {
	const query = useQuery();
	const { id, ...todo } = query.data;
	return { id, todo, ...query.data };
}

function DestructuredPropertyOfQueryResult() {
	const { data } = useQuery();
	const { id, ...todo } = data;
	return { id, todo, ...data };
}

function OtherHook() {
	const { mutate, ...rest } = useMutation();
	const mutation = useMutation();
	return { mutate, rest, ...mutation };
}

function ShadowedHook({ useQuery }) {
	const { data, ...rest } = useQuery();
	const query = useQuery();
	return { data, rest, ...query };
}

function ShadowedVariable() {
	const query = useQuery();
	return (query) => ({ ...query });
}

function OtherValues(props, values) {
	const { children, ...rest } = props;
	const [{ first, ...others }] = values;
	return { children, rest, first, others, ...props };
}

function Parameters({ data, ...rest }) {
	return [data, rest];
}
