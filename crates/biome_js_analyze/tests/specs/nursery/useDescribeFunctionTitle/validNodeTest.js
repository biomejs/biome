/* should not generate diagnostics */
import { describe } from "node:test";
import { myFunction } from "./myFunction";

describe("myFunction", () => {});

describe(myFunction.name, () => {});
