/* should generate diagnostics */
import { QueryClient } from "@tanstack/react-query";
import { QueryClient as Client } from "@tanstack/react-query";

function Component() {
	const queryClient = new QueryClient();
	return null;
}

function useHook() {
	const queryClient = new QueryClient();
	return queryClient;
}

function WithOptions() {
	const queryClient = new QueryClient({ defaultOptions: { queries: { retry: false } } });
	return null;
}

function CustomName() {
	let customName = new QueryClient();
	return null;
}

function Destructured() {
	const { defaultOptions } = new QueryClient();
	return null;
}

const Arrow = () => {
	const queryClient = new QueryClient();
	return null;
};

const useArrowHook = () => {
	const queryClient = new QueryClient();
	return queryClient;
};

const Expression = function () {
	const queryClient = new QueryClient();
	return null;
};

export default function DefaultExport() {
	const queryClient = new QueryClient();
	return null;
}

function Renamed() {
	const queryClient = new Client();
	return null;
}

function Parenthesized() {
	const queryClient = (new QueryClient());
	return null;
}

function WithComment() {
	const queryClient = new QueryClient(); // shared by the whole tree
	return null;
}

function InsideBlock({ enabled }) {
	if (enabled) {
		const queryClient = new QueryClient();
	}
	return null;
}
