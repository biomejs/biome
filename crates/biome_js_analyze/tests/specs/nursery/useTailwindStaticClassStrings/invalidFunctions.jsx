// should generate diagnostics
clsx(`bg-${color}`);
clsx({ [`text-${color}`]: active });
<div className={clsx(`bg-${color}`)} />;
<div title={clsx(`bg-${color}`)} />;
consume(clsx(`text-${color}`));
clsx(clsx(`border-${color}`));
cn(`bg-${color}`);
twMerge("text-" + color);
