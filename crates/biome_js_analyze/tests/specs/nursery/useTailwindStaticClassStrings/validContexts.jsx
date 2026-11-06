// should not generate diagnostics
<div className="[broken" />;
<div className={`[broken`} />;
clsx("[broken");
<div ClassName={`bg-${color}`} />;
other(`bg-${color}`);
// biome-ignore lint/nursery/useTailwindStaticClassStrings: Generated elsewhere.
cn(`bg-${color}`);
