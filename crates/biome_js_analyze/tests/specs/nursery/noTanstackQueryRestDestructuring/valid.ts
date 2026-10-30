/* should not generate diagnostics */
import type { useQuery } from "@tanstack/react-query";
import { type useSuspenseQuery } from "@tanstack/react-query";

declare const typedQuery: typeof useQuery;

function TypeOnlyImports() {
	const { data, ...rest } = useQuery();
	const { data: suspended, ...others } = useSuspenseQuery();
	return [data, rest, suspended, others, typedQuery];
}
