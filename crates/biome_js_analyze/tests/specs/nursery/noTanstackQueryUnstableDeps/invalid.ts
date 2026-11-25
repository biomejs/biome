/* should generate diagnostics */
import { useEffect } from "react";
import { useMutation, useQuery } from "@tanstack/vue-query";
import { useQuery as usePreactQuery } from "@tanstack/preact-query";

function Component() {
	const query = useQuery({ queryKey: ["a"], queryFn }) as unknown;
	const mutation = useMutation({ mutationFn })!;
	const preactQuery = usePreactQuery({ queryKey: ["a"], queryFn });

	useEffect(() => {}, [query as unknown, mutation!, preactQuery satisfies unknown]);
}
