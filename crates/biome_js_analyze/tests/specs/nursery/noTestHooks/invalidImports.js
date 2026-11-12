/* should generate diagnostics */
import { beforeEach, test } from "vitest";
import { before } from "node:test";
import { afterEach as cleanup } from "@jest/globals";
import { "afterAll" as done } from "bun:test";
import { after as beforeAll } from "mocha";

beforeEach(() => {});
before(() => {});
test.afterAll(() => {});
cleanup(() => {});
done(() => {});
beforeAll(() => {});
