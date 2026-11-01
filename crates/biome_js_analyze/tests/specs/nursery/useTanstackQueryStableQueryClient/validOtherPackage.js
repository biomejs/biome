/* should not generate diagnostics */
import { QueryClient as SolidQueryClient } from "@tanstack/solid-query";
import { QueryClient } from "other-library";

function Component() {
	const queryClient = new QueryClient();
	return null;
}

function SolidComponent() {
	const queryClient = new SolidQueryClient();
	return null;
}
