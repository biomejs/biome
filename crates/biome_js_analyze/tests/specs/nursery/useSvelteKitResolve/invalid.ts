import { goto } from "$app/navigation";
import type { Pathname } from "$app/types";

type ResolvedPathname = string;

function navigate(path: Pathname) {
	goto(path);
}

function navigateLocal(path: ResolvedPathname) {
	goto(path);
}
