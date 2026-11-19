// should generate diagnostics
<div className={`p-4 ${classes} text-white`} />;
<div className={"p-4 " + classes} />;
<div className={classes + " text-white"} />;
<div className={classes + " " + "text-white"} />;
<div className={"bg-red-500" + ""} />;
<div className={"" + "bg-red-500"} />;
<div className={"p-4\n" + classes} />;
<div className={`p-4\n${classes}`} />;
<div className={`${classes}`} />;
<div className={`${"p-4"} ${"text-white"}`} />;
<div className={first + second} />;
clsx(`p-4 ${classes}`);
clsx("p-4 " + "text-white");
clsx("p-4", `bg-${color}`, `text-${size}`);
<div className={active ? "p-4" : `bg-${color}`} />;
