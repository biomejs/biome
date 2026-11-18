/* should not generate diagnostics */
import {
	useInfiniteQuery,
	useQueries,
	useQuery,
	useSuspenseInfiniteQuery,
	useSuspenseQueries,
	useSuspenseQuery,
} from "other-package";
import useDefaultQuery from "@tanstack/react-query";
import { useQuery as useTableQuery } from "@tanstack/react-table";

function Query() {
	const { data, ...rest } = useQuery();
	const query = useQuery();
	return { data, rest, ...query };
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

function DefaultImport() {
	const { data, ...rest } = useDefaultQuery();
	return rest;
}

function OtherTanstackPackage() {
	const { data, ...rest } = useTableQuery();
	return rest;
}
