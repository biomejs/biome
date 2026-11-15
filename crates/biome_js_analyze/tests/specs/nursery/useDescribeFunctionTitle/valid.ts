/* should not generate diagnostics */
import type { typeOnly } from "./typeOnly";
import { type inlineTypeOnly } from "./inlineTypeOnly";
import type defaultTypeOnly from "./defaultTypeOnly";
import type * as namespaceTypeOnly from "./namespaceTypeOnly";

declare const declaredFunction: () => unknown;

describe("typeOnly", () => {});

describe("inlineTypeOnly", () => {});

describe("defaultTypeOnly", () => {});

describe("namespaceTypeOnly", () => {});

describe(namespaceTypeOnly.name, () => {});

describe("declaredFunction", () => {});
