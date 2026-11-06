import { goto, pushState, replaceState } from "$app/navigation";
import * as navigation from "$app/navigation";
import { goto as navigate } from "$app/navigation";
import { resolve, base } from "$app/paths";

goto("/foo");
goto(`/foo`);
goto("https://svelte.dev");
goto("");
goto("/foo" + resolve("/bar"));
goto(resolve("/foo") + "/bar");
goto(base + "/foo");
goto(condition ? resolve("/foo") : "/bar");
goto((("/foo")));

pushState("/foo", {});
replaceState("/foo", {});

navigation.goto("/foo");
navigate("/foo");

const path = "/foo";
goto(path);

const indirectPath = path;
goto(indirectPath);

let uninitializedPath;
goto(uninitializedPath);

function navigateTo(path) {
	goto(path);
}
