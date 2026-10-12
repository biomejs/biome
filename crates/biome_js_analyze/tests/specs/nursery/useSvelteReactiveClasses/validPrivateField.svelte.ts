/* should not generate diagnostics */
export class Cache {
	private entries = new Map<string, number>();
	#keys = new Set<string>();
}
