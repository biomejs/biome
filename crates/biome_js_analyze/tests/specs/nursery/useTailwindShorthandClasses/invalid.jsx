export function Component() {
	return (
		<>
			<div className="w-4 h-4" />
			<div className="hover:overflow-hidden hover:text-ellipsis hover:whitespace-nowrap" />
			<div className={clsx("w-4 h-4")} />
			<div className={tw.div`overflow-hidden text-ellipsis whitespace-nowrap`} />
			<div className="scale-x-50 scale-y-50 scale-z-50" />
			<div className="content-center justify-center" />
			<div className="!border-x !border-y" />
			<div className="!content-center !justify-center" />
		</>
	);
}
