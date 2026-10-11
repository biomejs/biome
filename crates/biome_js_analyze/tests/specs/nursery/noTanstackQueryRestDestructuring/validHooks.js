/* should not generate diagnostics */
import {
	useQuery,
	useInfiniteQuery,
	useSuspenseQuery,
	useSuspenseInfiniteQuery,
	useQueries,
	useSuspenseQueries,
} from "@tanstack/react-query";

function NotCaptured() {
	useQuery();
	useInfiniteQuery();
	useSuspenseQuery();
	useSuspenseInfiniteQuery();
	useQueries([]);
	useSuspenseQueries([]);
	return;
}

function NotDestructured() {
	const query = useQuery();
	const infiniteQuery = useInfiniteQuery();
	const suspenseQuery = useSuspenseQuery();
	const suspenseInfiniteQuery = useSuspenseInfiniteQuery();
	const queries = useQueries([]);
	const suspenseQueries = useSuspenseQueries([]);
	return;
}

function DestructuredWithoutRest() {
	const { data, isLoading, isError } = useQuery();
	const { data: infiniteData, isLoading: infiniteLoading } = useInfiniteQuery();
	const { data: suspenseData } = useSuspenseQuery();
	const { data: suspenseInfiniteData } = useSuspenseInfiniteQuery();
	return;
}

function ArrayWithoutObjectRest() {
	const [query1, { data, isLoading }, , ...others] = useQueries([
		{ queryKey: ["key1"], queryFn: () => {} },
		{ queryKey: ["key2"], queryFn: () => {} },
		{ queryKey: ["key3"], queryFn: () => {} },
		{ queryKey: ["key4"], queryFn: () => {} },
		{ queryKey: ["key5"], queryFn: () => {} },
	]);
	const [suspenseQuery1, { data: suspenseData }] = useSuspenseQueries([
		{ queryKey: ["key1"], queryFn: () => {} },
		{ queryKey: ["key2"], queryFn: () => {} },
	]);
	return;
}

function StoredWithoutRestOrSpread() {
	const query = useQuery();
	const { data } = query;
	const value = query.data;
	return { data, value };
}
