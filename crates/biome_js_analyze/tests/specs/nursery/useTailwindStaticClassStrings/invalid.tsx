// should generate diagnostics
<div className={`bg-${color}` as string} />;
<div className={("text-" + color) satisfies string} />;
<div className={("border-" + color)!} />;
