/* should not generate diagnostics */
import { importedCleanup } from "./cleanup";
promise.finally(importedCleanup);

type Callback = () => void;
promise.finally((() => {}) as Callback);
