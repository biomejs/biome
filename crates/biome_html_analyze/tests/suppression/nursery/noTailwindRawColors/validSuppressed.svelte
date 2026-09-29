<!-- should not generate diagnostics -->
<script>
// biome-ignore lint/nursery/noTailwindRawColors: This component displays the palette.
const classes = clsx("bg-pink-500");
</script>
<!-- biome-ignore lint/nursery/noTailwindRawColors: This component displays the palette. -->
<div class="bg-pink-500"></div>

<div class={/* biome-ignore lint/nursery/noTailwindRawColors: This component displays the palette. */
"bg-pink-500"}></div>

<div class={/* biome-ignore lint/nursery/noTailwindRawColors: This component displays the palette. */
["bg-pink-500", active && "text-red-500"]}></div>

<div class={/* biome-ignore lint/nursery/noTailwindRawColors: This component displays the palette. */
{ "bg-pink-500": active }}></div>

<div class={/* biome-ignore lint/nursery/noTailwindRawColors: This component displays the palette. */
active ? "bg-pink-500" : "text-red-500"}></div>

<div class={/* biome-ignore lint/nursery/noTailwindRawColors: This component displays the palette. */
`bg-pink-500 ${extra}`}></div>

<div class={/* biome-ignore lint/nursery/noTailwindRawColors: This component displays the palette. */
clsx("bg-pink-500", { "text-red-500": active })}></div>
