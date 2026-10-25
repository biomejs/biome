/* should not generate diagnostics */
import type { Options } from "./options";

type PrivateType = Options;
export type PublicType = PrivateType;

// Without this empty export, PrivateType would also be exported.
export {};
