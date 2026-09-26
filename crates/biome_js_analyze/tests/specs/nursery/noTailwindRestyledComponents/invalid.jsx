/* should generate diagnostics */
<Button className="rounded-none" />;
<Button class="text-sm font-bold" />;
<UI.Button className="hover:!rounded-none border-t-2" />;
<Button className="dark:rounded-none!" ></Button>;
<my-button className="font-bold shadow" />;
<Button className={"font-bold"} />;
<Button className={active ? "text-sm" : "text-lg"} />;
<Button className={active && "opacity-50"} />;
<Button className={cn("mt-4", active && "rounded-none")} />;
<Button className={clsx({ "rounded-none": active, shadow: active })} />;
<Button className={`mt-4 ${extra} text-sm`} />;
<Button className="[font-size:14px] hover:[border-radius:0]" />;
<Button className="[&>span]:bg-[url('a:b')]" />;
<Button className="café rounded-none" />;
