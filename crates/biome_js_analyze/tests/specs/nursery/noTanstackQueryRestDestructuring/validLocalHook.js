/* should not generate diagnostics */
function useQuery() {
	return { data: 1, isError: false };
}

function Component() {
	const { data, ...rest } = useQuery();
	const query = useQuery();
	return { data, rest, ...query };
}

function Global() {
	const { data, ...rest } = useSuspenseQuery();
	return rest;
}
