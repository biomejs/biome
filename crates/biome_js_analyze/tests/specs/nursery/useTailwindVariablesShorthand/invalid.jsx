// should generate diagnostics
<div className="bg-[var(--brand)]" />;
<div className="text-red-500 hover:bg-[var(--brand)]! p-4" />;
<div className="bg-[__var(--brand)__] bg-[var(--Brand)]" />;
<div className="bg-[var(--brand,_#000)] h-[var(--header,calc(100%_-_1rem))]" />;
<div className="bg-[var(--brand,var(--secondary))]" />;
<div className="bg-red-500/[var(--opacity)] bg-[var(--brand)]/[var(--opacity)]" />;
<div className="-mt-[var(--gap)] !p-[var(--gap)]" />;
<div className={'w-[var(--width)]'} />;
<div className={active ? "bg-[var(--brand)]" : "bg-[var(--muted)]"} />;
<div className={`p-4 ${extra} bg-[var(--brand)]`} />;
<div className={clsx("bg-[var(--brand)]", { "text-[var(--ink)]": active })} />;
<div className="
  bg-[var(--brand)]
  text-[var(--ink)]
" />;
