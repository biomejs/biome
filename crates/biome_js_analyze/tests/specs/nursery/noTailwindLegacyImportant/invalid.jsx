// should generate diagnostics
<div className="!flex" />;
<div className="p-4 hover:!bg-red-500/50 md:focus:!text-lg" />;
<div className="!-m-4 ![color:red] !bg-(--brand)" />;
<div className={'!grid'} />;
<div className={active ? "!block" : "!hidden"} />;
<div className={clsx("!flex", { "!underline": active })} />;
<div className={`p-4 ${extra} !flex`} />;
<div className={`!flex${suffix}`} />;
<div className={"!flex" + suffix} />;
<div className={prefix + "!flex"} />;
