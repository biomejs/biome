/* should generate diagnostics */
export const typed = query(async (ctx: QueryCtx): Promise<string[]> => []);

export const typedArgs = mutation(async (ctx: MutationCtx, args: { body: string }) => {});
