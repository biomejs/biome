/* should not generate diagnostics */

const foo = cn(m === "w-2 h-2" ? "bg-red-800" : "bg-red-400");
cn("w-2 h-2" ? "bg-red-800" : "bg-red-400");
cn((m === "w-2 h-2" && n === "p-2 p-2") ? "bg-red-800" : "bg-red-400");
cn((flag ? "w-2 h-2" : "w-4 h-4") ? "bg-red-800" : "bg-red-400");
<div className={m === "w-2 h-2" ? "bg-red-800" : "bg-red-400"} />;
<div className={m === `w-2 h-2` ? "bg-red-800" : "bg-red-400"} />;
