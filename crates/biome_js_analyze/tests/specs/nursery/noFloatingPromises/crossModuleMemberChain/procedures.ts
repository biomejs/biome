declare const trpc: unknown;

export const protectedProcedure = trpc.baseProcedure.use;
