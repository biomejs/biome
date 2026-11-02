/* should generate diagnostics */
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

function Annotated() {
	const queryClient: QueryClient = new QueryClient();
	return <QueryClientProvider client={queryClient} />;
}

function Asserted() {
	const queryClient = new QueryClient() as QueryClient;
	return <QueryClientProvider client={queryClient} />;
}

const Typed: React.FC = () => {
	const queryClient = new QueryClient() satisfies QueryClient;
	return <QueryClientProvider client={queryClient} />;
};
