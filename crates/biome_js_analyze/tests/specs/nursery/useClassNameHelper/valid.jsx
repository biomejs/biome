/* should not generate diagnostics */
<div className="px-4 bg-blue-500" />;
<div className={`px-4 bg-blue-500`} />;
<div className={(`px-4 bg-blue-500`)} />;
<div className={cn("px-4", isActive && "bg-blue-500", !isActive && "bg-gray-500")} />;
<div className={clsx("px-4", isActive && "bg-blue-500", !isActive && "bg-gray-500")} />;
<div className={isActive ? "bg-blue-500" : "bg-gray-500"} />;
<div className={cn("px-4", isActive ? "bg-blue-500" : "bg-gray-500")} />;
<div data-class={`px-4 ${isActive ? "bg-blue-500" : "bg-gray-500"}`} />;
