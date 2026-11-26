/* should generate diagnostics */
import { hash } from "./validNodeActions";
import "./validNodeActions";
import * as nodeActions from "./validNodeActions";
import defaultExport, { type Hash } from "./validNodeActions";
import { type Hash as OtherHash, hash as otherHash } from "./validNodeActions";
import { hash as hashWithExtension } from "./validNodeActions.ts";
import { runtime } from "./validNodeWithOtherDirectives";

export { hash as reexportedHash } from "./validNodeActions";
export * from "./validNodeActions";
export * as reexportedNodeActions from "./validNodeActions";
