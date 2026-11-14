/* should not generate diagnostics */
import { useQuery } from "@tanstack/react-query";

function useThing(svc, id) {
	return useQuery({
		queryKey: ["thing", id],
		queryFn: () => ({ part: svc.part, id }),
	});
}

function useOther() {
	const ignored = { run: () => Promise.resolve() };
	return useQuery({
		queryKey: ["thing"],
		queryFn: () => fetchThing(ignored),
	});
}
