// should generate diagnostics
<div className="bg-pink-500" />;
<div class="hover:text-white/80 dark:border-slate-950" />;
<div className={'ring-offset-black'} />;
<div className={active ? "bg-red-500" : "bg-blue-500"} />;
<div className={active && "bg-pink-500"} />;
<div className={(sideEffect(), "bg-pink-500")} />;
<div className={`bg-pink-500 ${extra} text-white`} />;
<div title="é" className="
  [&:nth-child(2)]:border-t-red-500/50!" />;
