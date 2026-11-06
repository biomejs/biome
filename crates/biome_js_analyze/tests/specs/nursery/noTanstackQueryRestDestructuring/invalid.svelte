<!-- should generate diagnostics -->
<script>
	import {
		createInfiniteQuery,
		createQueries,
		createQuery,
		createQuery as createData,
	} from "@tanstack/svelte-query";

	const { data, ...rest } = createQuery(() => ({ queryKey: ["todos"] }));

	const { data: pages, ...infiniteRest } = createInfiniteQuery(() => ({
		queryKey: ["pages"],
	}));

	const { data: renamed, ...renamedRest } = createData(() => ({
		queryKey: ["renamed"],
	}));

	const [first, { data: second, ...secondRest }] = createQueries(() => ({
		queries: [{ queryKey: ["first"] }, { queryKey: ["second"] }],
	}));

	const query = createQuery(() => ({ queryKey: ["users"] }));
	const { data: users, ...usersRest } = query;
	const copy = { ...query, users: query.data };

	const infiniteQuery = createInfiniteQuery(() => ({ queryKey: ["posts"] }));
	const infiniteCopy = { ...infiniteQuery };
</script>

<p>{data} {rest} {pages} {infiniteRest} {renamed} {renamedRest}</p>
<p>{first} {second} {secondRest} {users} {usersRest} {copy} {infiniteCopy}</p>
