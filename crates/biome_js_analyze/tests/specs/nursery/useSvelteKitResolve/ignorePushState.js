/* should not generate diagnostics */
import { goto, pushState, replaceState } from "$app/navigation";

pushState("/foo", {});
