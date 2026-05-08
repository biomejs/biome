/* should not generate diagnostics */

export function Component() {
	return (
		<>
			<div className="size-4" />
			<div className="hover:truncate" />
			<div className={clsx("size-4")} />
			<div className={tw.div`truncate`} />
		</>
	);
}
