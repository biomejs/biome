// should not generate diagnostics
const color = "bg-pink-500";
other("text-white");
clsx("bg-primary", { "text-muted-foreground": active });
cn("hover:border-brand-500/50");
tw`ring-current`;
clsx(`bg-primary`);
cn(`bg-pink-500${suffix}`);
cn(`${prefix}bg-pink-500`);
other(`bg-pink-500`);
cn(`bg-${color}`);
cn(`${prefix}h-4 w-4`);
