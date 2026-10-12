// should generate diagnostics
cva("p-4", { base: `bg-${color}` });
cva("p-4", { variants: { size: { sm: `px-${size}` } } });
cva("p-4", { variants: { size: { sm: ["p-2", "text-" + tone] } } });
cva("p-4", { compoundVariants: [{ size: "sm", class: `m-${space}` }] });
tv({ slots: { base: `flex-${direction}` } });
tv({ variants: { size: { sm: { base: `gap-${gap}` } } } });
tv({ compoundSlots: [{ slots: ["base"], className: `w-${width}` }] });
cva(`bg-${color}`, { variants: {} });
