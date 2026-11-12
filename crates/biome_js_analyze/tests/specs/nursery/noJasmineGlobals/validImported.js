/* should not generate diagnostics */
import jasmine, { fail, pending, spyOn, spyOnProperty } from "./test-helpers";

jasmine.any(Number);
jasmine.DEFAULT_TIMEOUT_INTERVAL = 5000;
fail();
pending();
spyOn(some, "object");
spyOnProperty(some, "object");
