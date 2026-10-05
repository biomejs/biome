/* should not generate diagnostics */

import { load } from "./load";

export function wrappedLoad(value: number) {
	return load(value);
}
