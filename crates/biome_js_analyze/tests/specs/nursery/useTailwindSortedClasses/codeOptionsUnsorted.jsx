<>
	{/* attributes */}
	{/* SHOULD emit diagnostics (class/className attributes supported by default) */}
	<div class="px-2 foo p-4 bar" />
	<div className="px-2 foo p-4 bar" />
	{/* SHOULD emit diagnostics (customClassAttribute attribute specified in options) */}
	<div customClassAttribute="px-2 foo p-4 bar" />
	{/* SHOULD NOT emit diagnostics (notClassAttribute attribute NOT specified in options) */}
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
</>;

// functions
/* SHOULD emit diagnostics (functions specified in options) */
clsx("px-2 foo p-4 bar");
tw`px-2 foo p-4 bar`;
tw.div`px-2 foo p-4 bar`;
notClassFunction("px-2 foo p-4 bar");
notTemplateFunction`px-2 foo p-4 bar`;
notTemplateFunction.div`px-2 foo p-4 bar`;

// nested values
/* SHOULD emit diagnostics (class attribute supported by default) */
<div class={"px-2 foo p-4 bar"} />;
<div class={["px-2 foo p-4 bar"]} />;
<div
	class={{
		"px-2 foo p-4 bar": [
			"px-2 foo p-4 bar",
			{ "px-2 foo p-4 bar": "px-2 foo p-4 bar", custom: ["px-2 foo p-4 bar"] },
		],
	}}
/>;
/* SHOULD emit diagnostics (clsx function specified in options) */
clsx(["px-2 foo p-4 bar"]);
clsx({
	"px-2 foo p-4 bar": [
		"px-2 foo p-4 bar",
		{ "px-2 foo p-4 bar": "px-2 foo p-4 bar", custom: ["px-2 foo p-4 bar"] },
	],
});
