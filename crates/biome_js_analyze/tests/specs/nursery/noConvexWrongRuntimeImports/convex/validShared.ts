/* should not generate diagnostics */
export function formatAmount(amount: number): string {
	return `$${amount.toFixed(2)}`;
}
