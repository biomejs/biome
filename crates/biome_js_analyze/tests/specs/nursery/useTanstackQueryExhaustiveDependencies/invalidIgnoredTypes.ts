/* should generate diagnostics */
import { useQuery } from "@tanstack/react-query";

interface MyService {
	baseUrl: string;
}

function useData(service: MyService) {
	return useQuery({
		queryKey: ["data"],
		queryFn: () => service.baseUrl,
	});
}

interface OtherService {
	baseUrl: string;
}

function useShadowed() {
	const svc: Svc = { baseUrl: "allowed" };
	if (true) {
		const svc: OtherService = { baseUrl: "other" };
		return useQuery({
			queryKey: ["thing"],
			queryFn: () => svc.baseUrl,
		});
	}
	return null;
}
