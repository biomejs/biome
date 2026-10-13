/* should generate diagnostics */
import { useQueries, useQuery } from "@tanstack/react-query";

function AsExpression() {
	const { data, ...rest } = useQuery() as { data: string[]; isPending: boolean };
	return rest;
}

function NonNullAssertion() {
	const { data, ...rest } = useQuery()!;
	return rest;
}

function TypeArguments() {
	const { data, ...rest } = useQuery<string[]>();
	return rest;
}

function AnnotatedVariable() {
	const query: { data: string[] } = useQuery();
	const { data, ...rest } = query;
	return rest;
}

function SpreadOfAsExpression() {
	const query = useQuery();
	return { ...(query as object) };
}

function Queries() {
	const [{ data, ...rest }] = useQueries([]) as { data: string[] }[];
	return rest;
}
