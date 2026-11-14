/* should generate diagnostics */
import { useQuery } from "@tanstack/react-query";

function useAsConst(dep: string) {
	return useQuery({
		queryKey: ["thing"] as const,
		queryFn: () => dep,
	});
}

function useKeyVariable(dep: string) {
	const key = ["thing"];
	return useQuery({
		queryKey: key,
		queryFn: () => dep,
	});
}

const useFactory = (num: number) =>
	useQuery({
		queryKey: keys.foo(),
		queryFn: () => Promise.resolve(num),
	});

function useNonNull(data?: { address: string }) {
	return useQuery({
		queryKey: ["query"],
		queryFn: () => sendQuery(data!.address),
	});
}
