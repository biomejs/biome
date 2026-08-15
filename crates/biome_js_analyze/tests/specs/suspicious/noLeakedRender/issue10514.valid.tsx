/* should not generate diagnostics */
type User = { readonly name: string };
type State = "WAIT" | "PICK" | "PACK";

type Props = {
	readonly showTrigger?: boolean;
	readonly isAdmin: boolean;
	readonly error: Error | null;
	readonly selectedUser: User | null;
	readonly title: "Home" | "Settings" | undefined;
	readonly onExportData: (() => void) | undefined;
	readonly state: State;
	readonly isMobile: boolean;
	readonly decorations: { readonly kind: string };
};

function FalsePositiveCases({
	showTrigger = true,
	isAdmin,
	error,
	selectedUser,
	title,
	onExportData,
	state,
	isMobile,
	decorations,
}: Props) {
	return (
		<>
			{showTrigger && <button type="button">Open</button>}
			{isAdmin && <section>Admin tools</section>}
			{error && <p>{error.message}</p>}
			{selectedUser && <span>{selectedUser.name}</span>}
			{title && <h1>{title}</h1>}
			{onExportData && (
				<button onClick={onExportData} type="button">
					Export
				</button>
			)}
			<p>{state === "WAIT" ? "Move to WAIT" : state}</p>
			{isMobile ? null : decorations}
		</>
	);
}
