/* should not generate diagnostics */
import { useInfiniteQuery, infiniteQueryOptions } from "@tanstack/react-query";
import { useInfiniteQuery as other } from "other-package";

useInfiniteQuery({
	queryKey: ["a"],
	queryFn: () => fetchA(),
	initialPageParam: 0,
	getPreviousPageParam: (p) => p.prev,
	getNextPageParam: (p) => p.next,
});

infiniteQueryOptions({
	getNextPageParam: (p) => p.next,
});

useInfiniteQuery(options);
useInfiniteQuery();

other({
	getNextPageParam,
	queryFn,
});

function local() {
	const useInfiniteQuery = () => {};
	useInfiniteQuery({ getNextPageParam, queryFn });
}

import * as rq from "@tanstack/react-query";
rq.useInfiniteQuery({ getNextPageParam, queryFn });
