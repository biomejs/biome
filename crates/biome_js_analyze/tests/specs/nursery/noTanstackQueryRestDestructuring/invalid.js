/* should generate diagnostics */
import {
	useInfiniteQuery,
	useQueries,
	useQuery,
	useQuery as useData,
	useSuspenseInfiniteQuery,
	useSuspenseQueries,
	useSuspenseQuery,
} from "@tanstack/react-query";
import { useQuery as useVueQuery } from "@tanstack/vue-query";

function Query() {
	const { data, ...rest } = useQuery();
	return rest;
}

function InfiniteQuery() {
	const { data, ...rest } = useInfiniteQuery();
	return rest;
}

function SuspenseQuery() {
	const { data, ...rest } = useSuspenseQuery();
	return rest;
}

function SuspenseInfiniteQuery() {
	const { data, ...rest } = useSuspenseInfiniteQuery();
	return rest;
}

function Queries() {
	const [query1, { data, ...rest }] = useQueries([
		{ queryKey: ["key1"], queryFn: () => {} },
		{ queryKey: ["key2"], queryFn: () => {} },
	]);
	return rest;
}

function SuspenseQueries() {
	const [query1, { data, ...rest }] = useSuspenseQueries([
		{ queryKey: ["key1"], queryFn: () => {} },
		{ queryKey: ["key2"], queryFn: () => {} },
	]);
	return rest;
}

function EveryQueryInList() {
	const [{ ...first }, , { ...third }] = useQueries([]);
	return [first, third];
}

function OnlyRest() {
	const { ...everything } = useQuery();
	return everything;
}

function ParenthesizedCall() {
	const { data, ...rest } = (useQuery());
	return rest;
}

function RenamedImport() {
	const { data, ...rest } = useData();
	return rest;
}

function OtherTanstackQueryPackage() {
	const { data, ...rest } = useVueQuery();
	return rest;
}

function DestructuredVariable() {
	const query = useQuery();
	const { data, ...rest } = query;
	return rest;
}

function SpreadInReturn() {
	const query = useQuery();
	return { ...query, data: query.data[0] };
}

function SpreadInObject() {
	const query = useQuery();
	const result = { ...query, data: query.data[0] };
	return result;
}

function SpreadOfParenthesizedVariable() {
	const query = useSuspenseQuery();
	return { ...(query) };
}

function SpreadOfCall() {
	return { ...useQuery() };
}

function SpreadInNestedFunction() {
	const query = useInfiniteQuery();
	return () => ({ ...query });
}
