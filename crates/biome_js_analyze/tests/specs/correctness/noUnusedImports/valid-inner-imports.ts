declare module "mod" {
    import type { Ns } from "other";
    export const C: { prop: Ns.prop };
}