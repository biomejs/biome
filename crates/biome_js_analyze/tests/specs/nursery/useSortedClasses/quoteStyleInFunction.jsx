// functions
tw("content-[''] absolute");
tw({ base: "content-[''] absolute" });
tw({ variant: { dark: "content-[''] absolute", light: "flex gap-2 p-4 m-2 [&_svg:not([class*='size-'])]:w-4 items-center" } });
tw('flex gap-2 p-4 m-2 [&_svg:not([class*="size-"])]:w-4 items-center');

// function in jsx attribute
<div class={tw("content-[''] absolute")}>Hello</div>;
<div class={tw({ base: "content-[''] absolute" })}>Hello</div>;
<div class={tw({ variant: { dark: "content-[''] absolute", light: "flex gap-2 p-4 m-2 [&_svg:not([class*='size-'])]:w-4 items-center" } })}>Hello</div>;
<div class={tw('flex gap-2 p-4 m-2 [&_svg:not([class*="size-"])]:w-4 items-center')} />;
