/* should not generate diagnostics */
array.splice(0 as const, 2);
(object?.array as string[]).splice(index, 0);
((object?.array as { items: string[] }).items).splice(index, 0);
(object?.array satisfies string[]).splice(0);
const removed: unknown[] = array.splice(0 as const);
const allRemoved: unknown[] = array.splice(0 as const, array.length);
