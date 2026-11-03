/* should not generate diagnostics */
declare const big: bigint;
~~big;
parseInt(String(big), 10);

declare const ten: 10n;
~~ten;

function truncate(value: bigint) {
	return ~~value;
}
