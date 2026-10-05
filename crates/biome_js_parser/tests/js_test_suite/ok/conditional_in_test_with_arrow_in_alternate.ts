declare function tool(x: { f: (o: { output: { error: string } | { items: { k: string }[] } }) => unknown }): unknown;
export const t = tool({
	f: ({ output }) => ({ v: "error" in output ? output : { items: output.items.map((i) => ({ k: i.k })) } }),
});
