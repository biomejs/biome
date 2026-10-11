/* should generate diagnostics */
import { useQuery, useInfiniteQuery, useSuspenseQuery, useSuspenseInfiniteQuery } from "@tanstack/react-query";
import { useQuery as useAliasedQuery } from "@tanstack/react-query";
import { useQuery as useVueQuery } from "@tanstack/vue-query";

const { data, ...rest } = useQuery({ queryKey: ["todos"], queryFn: fetchTodos });
const { data: pages, ...infinite } = useInfiniteQuery({ queryKey: ["todos"], queryFn: fetchTodos });
const { ...suspense } = useSuspenseQuery({ queryKey: ["todos"], queryFn: fetchTodos });
const { ...suspenseInfinite } = useSuspenseInfiniteQuery({ queryKey: ["todos"], queryFn: fetchTodos });
const { data: aliased, ...aliasedRest } = useAliasedQuery({ queryKey: ["todos"], queryFn: fetchTodos });
const { data: parenthesized, ...parenthesizedRest } = (useQuery)({ queryKey: ["todos"], queryFn: fetchTodos });

function Component() {
	const { data: nested, ...nestedRest } = useQuery({ queryKey: ["nested"], queryFn: fetchTodos });
	return nested;
}
const { data: vue, ...vueRest } = useVueQuery({ queryKey: ["vue"], queryFn: fetchTodos });
