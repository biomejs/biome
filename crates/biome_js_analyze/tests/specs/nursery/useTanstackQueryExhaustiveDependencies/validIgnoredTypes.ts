/* should not generate diagnostics */
import { useQuery } from "@tanstack/react-query";

interface Svc {
	part: { baseUrl: string };
}

function useReference(svc: Svc, id: string) {
	return useQuery({
		queryKey: ["thing", id],
		queryFn: () => ({ baseUrl: svc.part.baseUrl, id }),
	});
}

function useUnion(value: AllowedType | OtherType, id: string) {
	return useQuery({ queryKey: ["thing", id], queryFn: () => fetchThing(value, id) });
}

function useIntersection(value: AllowedType & OtherType, id: string) {
	return useQuery({ queryKey: ["thing", id], queryFn: () => fetchThing(value, id) });
}

function useArray(value: AllowedType[], id: string) {
	return useQuery({ queryKey: ["thing", id], queryFn: () => fetchThing(value, id) });
}

function useTuple(value: [AllowedType, string], id: string) {
	return useQuery({ queryKey: ["thing", id], queryFn: () => fetchThing(value, id) });
}

function useVariable(id: string) {
	const svc: Svc = createSvc();
	return useQuery({ queryKey: ["thing", id], queryFn: () => svc.part.baseUrl + id });
}
