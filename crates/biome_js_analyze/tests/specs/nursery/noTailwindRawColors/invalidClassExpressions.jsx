// should generate diagnostics
<div className={["bg-pink-500", [active && "hover:text-red-500/80"]]} />;
<div className={{ "border-slate-950": active }} />;
<div className={{ ["bg-pink-500"]: active }} />;
<div className={[...["bg-pink-500"]]} />;
<div className={{ ...{ "bg-pink-500": active } }} />;
<div className={classes ?? "bg-pink-500"} />;
<div className={`bg-primary ${active ? "bg-pink-500" : "text-red-500"}`} />;
clsx(`bg-pink-500 ${active ? "text-red-500" : "text-primary"}`);
