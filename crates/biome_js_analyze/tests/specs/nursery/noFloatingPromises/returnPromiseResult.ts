import type { PromisedResult } from "./promisedResult.ts";

function returnPromiseResult(): PromisedResult {
  return new Promise(resolve => resolve({ result: true }));
}

export { returnPromiseResult };
