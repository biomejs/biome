/* should not generate diagnostics */
import { QueryClient } from "@tanstack/react-query";
import { useEffect, useMemo, useRef, useState } from "react";

const sharedClient = new QueryClient();

function WithReactUseState() {
	const [queryClient] = React.useState(() => new QueryClient());
	return null;
}

function WithUseState() {
	const [queryClient] = useState(() => new QueryClient());
	return null;
}

function WithUseMemo() {
	const queryClient = useMemo(() => new QueryClient(), []);
	return null;
}

function WithAnyHook() {
	const [queryClient] = useAnything(() => new QueryClient());
	return null;
}

function createClient() {
	const queryClient = new QueryClient();
	return queryClient;
}

function user() {
	const queryClient = new QueryClient();
	return queryClient;
}

async function ServerComponent() {
	const queryClient = new QueryClient();
	await queryClient.prefetchQuery({ queryKey: ["todos"], queryFn: fetchTodos });
	return null;
}

const AsyncArrow = async () => {
	const queryClient = new QueryClient();
	return null;
};

function WithEffect() {
	useEffect(() => {
		const queryClient = new QueryClient();
		queryClient.clear();
	}, []);
	return null;
}

function WithHandler() {
	const handleClick = () => {
		const queryClient = new QueryClient();
		queryClient.clear();
	};
	return null;
}

class Provider {
	queryClient = new QueryClient();
}
