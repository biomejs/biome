/* should generate diagnostics */
<div className="flex grid" />;
<div className="flex w-full grid" />;
<div className="p-2 p-4" />;
<div className="p-2 p-[3px]" />;
<div className="m-2 -m-2" />;
<div className="text-red-500 text-blue-500/50" />;
<div className="flex flex!" />;
<div className="hover:flex hover:grid" />;
<div className="sm:hover:flex sm:hover:grid" />;
<div className="block inline flex" />;
<div className="flex flex grid" />;
<div className="flex p-2 grid p-4" />;
<div className="text-sm text-lg" />;
<div className="text-sm text-[14px]/6" />;
<div className="shadow-md/50 shadow-lg/20" />;
<div className="[--gap:1rem] [--gap:2rem]" />;
<div className="flex [display:grid]" />;
<div className="bg-linear-to-r bg-linear-to-b" />;
<div className="select-none select-text" />;
<div className='flex grid' />;
<div
	className="flex
		grid"
/>;
<div className={"flex grid"} />;
<div className={"flex grid " + extra} />;
<div className={extra + " flex grid"} />;
<div className="bg-red-500 bg-[#fff]" />;
<div className="font-bold font-(--weight)" />;
<div className={`flex grid ${extra}`} />;
<div className={clsx("flex grid", isActive && "p-2 p-4")} />;
<div className={clsx({ "flex grid": isActive })} />;
<div className={isActive ? "flex grid" : "p-2 p-4"} />;
const button = cva("flex grid", { variants: { size: { small: "p-2 p-4" } } });
const classes = tw`flex grid`;
