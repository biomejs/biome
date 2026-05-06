/* should not generate diagnostics */

export async function baseDoWork(id: string): Promise<void> {
	await new Promise((resolve) => setTimeout(resolve, 100));
}
