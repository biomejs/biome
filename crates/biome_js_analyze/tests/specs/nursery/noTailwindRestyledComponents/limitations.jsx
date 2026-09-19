<Button className={"rounded-none" && "mt-4"} />;
<Button className={clsx({ "mt-4": tone === "rounded-none" })} />;
<Button className={`custom${active ? "rounded-none" : "shadow-lg"}`} />;
<Button className={`${"rounded-none"}-custom`} />;
<Button className={`${extra}${"rounded-none"}`} />;
