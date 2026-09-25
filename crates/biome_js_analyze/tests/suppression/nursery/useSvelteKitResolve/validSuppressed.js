// should not generate diagnostics
import { goto } from "$app/navigation";

// biome-ignore lint/nursery/useSvelteKitResolve: the path is resolved by the caller.
goto("/foo");
