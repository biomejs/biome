/* should not generate diagnostics */
import { after } from "next/server";
import { before } from "lodash-es";
import { test as beforeAll } from "vitest";
import afterAll from "node:test";

const { beforeEach } = require("./helpers");
const afterEach = require("node:test");
const {
	hooks: { before: teardown },
} = require("mocha");

after(() => {});
before(2, () => {});
beforeAll("is a test", () => {});
afterAll(() => {});
beforeEach(() => {});
afterEach(() => {});
teardown(() => {});
