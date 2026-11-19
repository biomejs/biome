/* should not generate diagnostics */
import { goto, pushState, replaceState } from "$app/navigation";
import * as navigation from "$app/navigation";
import { resolve, asset } from "$app/paths";
import * as paths from "$app/paths";
import { resolve as resolvePath } from "$app/paths";

goto(resolve("/foo"));
goto(resolve("/blog/[slug]", { slug: "hello" }));
goto(asset("/favicon.png"));
goto(paths.resolve("/foo"));
goto(resolvePath("/foo"));
goto(condition ? resolve("/foo") : resolve("/bar"));
goto((resolve("/foo")));
navigation.goto(resolve("/foo"));

pushState(resolve("/foo"), {});
replaceState(resolve("/foo"), {});

// Shallow routing
pushState("", {});
replaceState("", {});
pushState(``, {});

const path = resolve("/foo");
goto(path);

const indirectPath = path;
goto(indirectPath);

// No arguments
goto();

// Functions that aren't imported from `$app/navigation`
function localGoto(path) {}
localGoto("/foo");
window.goto("/foo");
