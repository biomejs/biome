/* should not generate diagnostics */
import { useQuery } from "other-package";
import { useInfiniteQuery } from "other-package";
import { useSuspenseQuery } from "other-package";
import { useSuspenseInfiniteQuery } from "other-package";
import { useQueries } from "other-package";
import { useSuspenseQueries } from "other-package";

function Component() {
	const { data, ...rest } = useQuery();
	const { data: infiniteData, ...infiniteRest } = useInfiniteQuery();
	const { data: suspenseData, ...suspenseRest } = useSuspenseQuery();
	const { data: suspenseInfiniteData, ...suspenseInfiniteRest } = useSuspenseInfiniteQuery();
	const [query1, { data: queriesData, ...queriesRest }] = useQueries([]);
	const [suspenseQuery1, { data: suspenseQueriesData, ...suspenseQueriesRest }] = useSuspenseQueries([]);
	return;
}
