/* should generate diagnostics */
import { expect } from "chai";
import jest from "./jest-helpers";

// `expect` and `jest` aren't Jest's globals here, so no fix is offered.
expect(total).to.equal(jasmine.any(Number));
jasmine.DEFAULT_TIMEOUT_INTERVAL = 5000;
