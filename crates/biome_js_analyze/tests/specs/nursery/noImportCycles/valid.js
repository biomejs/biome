/* should not generate diagnostics */

import { foo } from "./invalidFoobar";

foo();

export function bar() {
  return 1;
}

export * as bar from "./valid"

import { foobar } from "./valid"

foobar.bar();
