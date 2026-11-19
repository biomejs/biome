/* should not generate diagnostics */

export function Component() {
	return (
		<>
			<div className="size-4" />
			<div className="hover:truncate" />
			<div className="rotate-x-45 rotate-y-45" />
			<div className="divide-x divide-y" />
			<div className="divide-x-4 divide-y-4" />
			<div className="content-normal justify-normal" />
			<div className="content-start justify-start" />
			<div className="!h-4 w-4" />
			<div className="h-4 !w-4" />
			<div className="w-sidebar h-sidebar" />
			<div className={clsx("size-4")} />
			<div className={tw.div`truncate`} />
			{custom("w-4 h-4")}
			{clsx(parse("w-4 h-4"))}
		</>
	);
}
