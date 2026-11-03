/* should generate diagnostics */
/// <reference path="./other.d.ts" />
/// <reference types="foo" />
declare module "bar" {
	import { Foo } from "foo";
}
