/* should generate diagnostics */
import { describe } from "vitest";
import { myFunction } from "./myFunction";

describe("myFunction", () => {});

describe.each([[1]])("myFunction", (value) => {});
