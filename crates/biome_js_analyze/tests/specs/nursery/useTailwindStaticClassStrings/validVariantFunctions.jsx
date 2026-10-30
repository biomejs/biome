// should not generate diagnostics
cva("p-4", { variants: { size: { sm: "px-2", lg: active ? "px-4" : "px-6" } } });
cva("p-4", { variants: { [`size-${name}`]: { sm: "px-2" } } });
cva("p-4", { defaultVariants: { size: `s${size}` } });
cva("p-4", { compoundVariants: [{ size: `s${size}`, class: "m-2" }] });
tv({ slots: { base: "flex", icon: "size-4" } });
clsx({ base: `bg-${color}` });
