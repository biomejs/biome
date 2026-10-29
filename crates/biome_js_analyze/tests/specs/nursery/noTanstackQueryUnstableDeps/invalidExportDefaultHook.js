/* should generate diagnostics */
import { useEffect } from "react";
import { useQuery } from "@tanstack/react-query";

export default function useTodos() {
	return useQuery({ queryKey: ["todos"], queryFn });
}

function Component() {
	const todos = useTodos();

	useEffect(() => {}, [todos]);
}
