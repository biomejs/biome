/* should generate diagnostics */
import { expect, jest } from "@jest/globals";

// `expect` and `jest` are imported, so the fix is still offered.
expect(total).toEqual(jasmine.any(Number));
jasmine.DEFAULT_TIMEOUT_INTERVAL = 5000;
