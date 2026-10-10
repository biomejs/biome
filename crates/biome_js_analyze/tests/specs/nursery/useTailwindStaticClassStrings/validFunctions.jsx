// should not generate diagnostics
clsx(active && "bg-red-500", "text-white");
clsx({ "bg-red-500": "prefix-" + value });
other(`bg-${color}`);
<div styles={"bg-" + color} />;
