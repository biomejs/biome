// should generate diagnostics
<div className={`bg-${color}`} />;
<div class={"text-" + color} />;
<div className={`${size}-4`} />;
<div className={`hover:bg-${color}-500`} />;
<div className={active ? `bg-${color}` : "bg-blue-500"} />;
<div className={active && "text-" + color} />;
<div className={["p-4", `bg-${color}`]} />;
<div className={{ [`bg-${color}`]: active }} />;
<div className={"bg-" + "red-500"} />;
<div className={"p-4 " + ("bg-" + color)} />;
<div className={(`text-${color}`)} />;
<div className={`é-${color}`} />;
<div className={"p-4 " + "bg-" + color} />;
<div className={"p-4 " + classes + " bg-" + color} />;
<div className={`bg-${active ? "red-500" : "blue-500"}`} />;
