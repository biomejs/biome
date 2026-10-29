/* should not generate diagnostics */
import { useEffect } from "react";
import { useQuery } from "./router";
import { useMutation } from "tanstack-query";
import { useQueries } from "@tanstack/react-table";

function Component() {
	const query = useQuery();
	const mutation = useMutation();
	const queries = useQueries();

	useEffect(() => {}, [query, mutation, queries]);
}
