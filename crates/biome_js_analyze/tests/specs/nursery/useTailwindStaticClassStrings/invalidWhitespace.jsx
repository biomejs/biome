// should generate diagnostics
<div className={"bg-red-500" + (active ? " text-white" : "")} />;
<div className={`bg-red-500${active ? " text-white" : ""}`} />;
<div className={`${active ? "text-white " : ""}bg-red-500`} />;
<div className={(active ? "text-white " : "") + "bg-red-500"} />;
