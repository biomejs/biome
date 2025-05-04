/* should not generate diagnostics */
import {test, expect} from "bun:test";

test("something", () => {
    expect("something").toBeTrue()
})
