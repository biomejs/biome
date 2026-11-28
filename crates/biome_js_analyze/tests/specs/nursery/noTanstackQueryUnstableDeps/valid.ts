/* should not generate diagnostics */
import { useEffect } from "react";
import type { useQuery } from "@tanstack/react-query";
import { type useMutation } from "@tanstack/react-query";

function Component() {
	const query = useQuery({ queryKey: ["a"], queryFn });
	const mutation = useMutation({ mutationFn });

	useEffect(() => {}, [query, mutation]);
}
