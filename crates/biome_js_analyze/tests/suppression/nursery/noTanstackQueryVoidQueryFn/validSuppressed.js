/* should not generate diagnostics */
import { useQuery } from "@tanstack/react-query";

useQuery({
	queryKey: ["test"],
	// biome-ignore lint/nursery/noTanstackQueryVoidQueryFn: test
	queryFn: async () => {
		await fetch("/api/test");
	},
});

useQuery({
	queryKey: ["test"],
	// biome-ignore lint/nursery/noTanstackQueryVoidQueryFn: test
	async queryFn() {
		await fetch("/api/test");
	},
});
