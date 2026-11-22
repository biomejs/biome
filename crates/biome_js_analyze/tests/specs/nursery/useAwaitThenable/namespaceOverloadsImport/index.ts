/* should not generate diagnostics */

import { act } from "./act";

export async function awaitImportedOverload(): Promise<void> {
    await act(async () => {});
}
