{
	/* SHOULD NOT emit diagnostics (classes are sorted correctly)
	- with double quotes outside and single quotes inside */
}
<div class="m-2 flex items-center gap-2 p-4 [&_svg:not([class*='size-'])]:w-4" />;

{
	/* SHOULD emit diagnostics (classes are not sorted correctly)
	- with double quotes outside and single quotes inside */
}
<div class="flex gap-2 p-4 m-2 [&_svg:not([class*='size-'])]:w-4 items-center" />;

{
	/* SHOULD emit diagnostics (classes are not sorted correctly)
	- with single quotes outside and double quotes inside */
}
<div class='flex gap-2 p-4 m-2 [&_svg:not([class*="size-"])]:w-4 items-center' />;
