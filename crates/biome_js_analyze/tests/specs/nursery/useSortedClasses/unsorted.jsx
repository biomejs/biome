<>
	{/* attributes */}
	{/* SHOULD emit diagnostics (class/className attributes supported by default) */}
	<div class="px-2 foo p-4 bar" />
	<div className="px-2 foo p-4 bar" />
	{/* SHOULD NOT emit diagnostics (custom attributes not specified in options) */}
	<div customClassAttribute="px-2 foo p-4 bar" />
	<div notClassAttribute="px-2 foo p-4 bar" />
	{/* utility sorting */}
	{/* SHOULD emit diagnostics (class attribute supported by default) */}
	<div class="text-center custom-style1 p-4 bg-blue-500 text-white foo rounded-lg shadow-lg" />
	<div class="flex custom-layout items-center justify-center h-screen bg-gray-200 bar text-lg font-bold" />
	<div class="grid custom-grid grid-cols-3 gap-4 p-6 m-6 border border-gray-300 shadow-md rounded-md" />
	<div class="absolute top-0 right-0 m-4 p-2 text-sm bg-red-600 text-white rounded-full custom-alert" />
	<div class="inline-block bar bg-green-300 text-green-800 p-2 rounded border border-green-500 custom-button" />
	<div class="flex-col custom-list space-y-4 p-6 bg-white shadow-md rounded-lg divide-y divide-gray-200" />
	<div class="relative overflow-hidden custom-background bg-no-repeat bg-cover h-64 w-full foo m-2" />
	<div class="underline custom-text foo text-2xl font-semibold my-2" />
	<div class="flex-wrap custom-container justify-between items-start bar bg-purple-200 p-5 text-purple-700" />
	<div class="gap-8 bg-indigo-100 text-indigo-900 p-3 border-l-4 border-indigo-500 custom-border" />
	{/* variant sorting */}
	{/* SHOULD emit diagnostics (arbitrary variants not supported yet) */}
	<div class="checked:text-center custom-style1 p-4 hover:bg-blue-500 focus:text-white foo hover:focus:rounded-lg shadow-lg" />
	<div class="flex valid:required:bg-gray-200 custom-layout items-center required:justify-center valid:h-screen bar first-letter:text-lg font-bold" />
	<div class="focus-within:hover:shadow-md grid custom-grid grid-cols-3 gap-4 p-6 m-6 focus-within:border focus-within:border-gray-300 optional:rounded-md" />
	<div class="lg:absolute sm:top-0 md:right-0 lg:m-4 md:p-2 xl:text-sm sm:bg-red-600 sm:text-white sm:rounded-full lg:custom-alert" />
	<div class="checked:sm:inline-block bar hover:md:bg-green-300 hover:lg:text-green-800 p-2 rounded border sm:checked:border-green-500 custom-button" />
	{/* TODO: arbitrary variant */}
	<div class="[&nth-child(2)]:[&nth-child(6)]:text-red-200 [&nth-child(2)]:text-red-300 group-hover:flex-col focus:bg-red-100 py-4 text-red-500 hover:focus:bg-red-200 has-[:visited]:flex print:text-red-50 [&nth-child(3)]:text-red-200  container" />
	<div class="text-black [&nth-child(2)]:focus:text-red-100 [&nth-child(1)]:group-first:text-red-100 [&nth-child(2)]:hover:focus:bg-red-900 group-first:text-yellow-400 [&nth-child(2)]:text-black focus:bg-sky-100 py-4 checked:visited:bg-yellow-300 hover:text-red-400" />
	<div class="group-aria-disabled:bg-red-50 group-[:visited]:text-red-400 group-target:font-bold" />
	<div class="group-has-[.custom-class]:focus:underline aria-[sort=ascending]:bg-red-300 group-aria-[sort=ascending]:text-yellow-200 has-[:checked]:focus:bg-yellow-300 text-red-400" />
</>;

// functions
/* SHOULD NOT emit diagnostics (functions not specified in options) */
clsx("px-2 foo p-4 bar");
tw`px-2 foo p-4 bar`;
tw.div`px-2 foo p-4 bar`;
notClassFunction("px-2 foo p-4 bar");
notTemplateFunction`px-2 foo p-4 bar`;
notTemplateFunction.div`px-2 foo p-4 bar`;

// nested values
/* SHOULD emit diagnostics (class attribute supported by default) */
<div class={"px-2 foo p-4 bar"} />;
<div class={`px-2 foo p-4 bar`} />;
<div class={["px-2 foo p-4 bar"]} />;
<div class={[`px-2 foo p-4 bar`]} />;
<div
	class={{
		"px-2 foo p-4 bar": [
			"px-2 foo p-4 bar",
			{ "px-2 foo p-4 bar": "px-2 foo p-4 bar", custom: ["px-2 foo p-4 bar"] },
		],
	}}
/>;
/* SHOULD NOT emit diagnostics (clsx function not specified in options) */
clsx(["px-2 foo p-4 bar"]);
clsx({
	"px-2 foo p-4 bar": [
		"px-2 foo p-4 bar",
		{ "px-2 foo p-4 bar": "px-2 foo p-4 bar", custom: ["px-2 foo p-4 bar"] },
	],
});
