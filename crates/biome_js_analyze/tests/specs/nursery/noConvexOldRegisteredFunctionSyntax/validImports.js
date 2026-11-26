/* should not generate diagnostics */
import { action } from "mobx";
import { query as cachedQuery } from "some-library";

// `action` comes from a package that isn't Convex.
export const setUser = action((user) => {
	store.user = user;
});

export const cached = cachedQuery(async () => {});
