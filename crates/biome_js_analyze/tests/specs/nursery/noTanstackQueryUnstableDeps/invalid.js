/* should generate diagnostics */
import * as React from "react";
import { useCallback, useEffect, useMemo as useMemoAlias } from "react";
import {
	useInfiniteQuery,
	useMutation,
	useQueries,
	useQuery,
	useQuery as useRenamedQuery,
	useSuspenseInfiniteQuery,
	useSuspenseQueries,
	useSuspenseQuery,
} from "@tanstack/react-query";

function DirectHooks() {
	const query = useQuery({ queryKey: ["a"], queryFn });
	const suspenseQuery = useSuspenseQuery({ queryKey: ["a"], queryFn });
	const infiniteQuery = useInfiniteQuery({ queryKey: ["a"], queryFn });
	const suspenseInfiniteQuery = useSuspenseInfiniteQuery({ queryKey: ["a"], queryFn });
	const queries = useQueries({ queries: [{ queryKey: ["a"], queryFn }] });
	const suspenseQueries = useSuspenseQueries({ queries: [{ queryKey: ["a"], queryFn }] });
	const mutation = useMutation({ mutationFn });
	const renamedQuery = useRenamedQuery({ queryKey: ["a"], queryFn });

	useEffect(() => {}, [query]);
	useEffect(() => {}, [suspenseQuery]);
	useEffect(() => {}, [infiniteQuery]);
	useEffect(() => {}, [suspenseInfiniteQuery]);
	useEffect(() => {}, [queries]);
	useEffect(() => {}, [suspenseQueries]);
	useCallback(() => mutation.mutate("hello"), [mutation]);
	useEffect(() => {}, [renamedQuery]);
}

function ReactHookForms() {
	const query = useQuery({ queryKey: ["a"], queryFn });

	React.useEffect(() => {}, [query]);
	React.useCallback(() => {}, [query]);
	React.useMemo(() => query.data, [query]);
	useMemoAlias(() => query.data, [query]);
	(useEffect)(() => {}, [(query)]);
}

function MultipleDependencies() {
	const query = useQuery({ queryKey: ["a"], queryFn });
	const mutation = useMutation({ mutationFn });

	useEffect(() => {}, [query.data, query, mutation]);
}

function ArrayDestructuring() {
	const [first, second = fallback, ...rest] = useQueries({
		queries: [
			{ queryKey: ["a"], queryFn },
			{ queryKey: ["b"], queryFn },
			{ queryKey: ["c"], queryFn },
		],
	});
	const [suspenseFirst] = useSuspenseQueries({ queries: [{ queryKey: ["a"], queryFn }] });

	useEffect(() => {}, [first, second, rest, suspenseFirst]);
}

function useArrowQuery() {
	return useQuery({ queryKey: ["a"], queryFn });
}

const useArrowMutation = () => useMutation({ mutationFn });

const useBlockArrowQuery = () => {
	const options = { queryKey: ["a"], queryFn };
	return useQuery(options);
};

const useFunctionExpressionQuery = function () {
	return useSuspenseQuery({ queryKey: ["a"], queryFn });
};

function CustomHooks() {
	const arrowQuery = useArrowQuery();
	const arrowMutation = useArrowMutation();
	const blockArrowQuery = useBlockArrowQuery();
	const functionExpressionQuery = useFunctionExpressionQuery();
	const laterQuery = useLaterQuery();

	useEffect(() => {}, [
		arrowQuery,
		arrowMutation,
		blockArrowQuery,
		functionExpressionQuery,
		laterQuery,
	]);
}

function useLaterQuery() {
	return useInfiniteQuery({ queryKey: ["a"], queryFn });
}
