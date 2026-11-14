/* should generate diagnostics */
import { useQuery } from "@tanstack/react-query";

function useThing(svc, other) {
	return useQuery({
		queryKey: ["thing"],
		queryFn: () => ({ svcPart: svc.part, otherX: other.x }),
	});
}
