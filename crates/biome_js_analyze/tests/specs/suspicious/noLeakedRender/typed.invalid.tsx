/* should generate diagnostics */
type Props = {
	readonly count: number;
	readonly label: string;
	readonly optionalCount: number | undefined;
	readonly nullableLabel: string | null;
	readonly selectedUser: { readonly name: string } | null;
};

function Component({
	count,
	label,
	optionalCount,
	nullableLabel,
	selectedUser,
}: Props) {
	return (
		<div>
			{count && <span>Count: {count}</span>}
			{label && <span>{label}</span>}
			{optionalCount && <span>Count: {optionalCount}</span>}
			{nullableLabel && <span>{nullableLabel}</span>}
			<p>{count ? "has count" : selectedUser}</p>
		</div>
	);
}
