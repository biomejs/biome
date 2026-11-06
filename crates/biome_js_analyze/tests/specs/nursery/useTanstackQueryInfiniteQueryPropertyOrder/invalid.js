/* should generate diagnostics */
import { useInfiniteQuery, useSuspenseInfiniteQuery as useData, infiniteQueryOptions } from "@tanstack/react-query";

useInfiniteQuery({
	queryKey: ["a"],
	getNextPageParam: (p) => p.next,
	queryFn: () => fetchA(),
	initialPageParam: 0,
});

useData({
	getNextPageParam: (p) => p.next,
	getPreviousPageParam: (p) => p.prev,
	queryFn() {},
});

infiniteQueryOptions({
	getPreviousPageParam,
	queryFn,
	getNextPageParam,
});

useInfiniteQuery({
	getNextPageParam,
	queryFn,
});

useInfiniteQuery(({
	// next page
	getNextPageParam,
	// fetcher
	queryFn,
}));

useInfiniteQuery({
	getNextPageParam,
	"queryFn": fetchA,
});

useInfiniteQuery({
	getNextPageParam,
	queryFn: a,
	queryFn: b,
});
