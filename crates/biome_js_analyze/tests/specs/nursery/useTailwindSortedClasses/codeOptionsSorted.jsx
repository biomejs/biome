<>
	{/* attributes */}
	<div class="foo bar p-4 px-2" />
	<div className="foo bar p-4 px-2" />
	<div customClassAttribute="foo bar p-4 px-2" />
	<div notClassAttribute="px-2 foo p-4 bar" />
	{/* utility sorting */}
	<div class="custom-style1 foo rounded-lg bg-blue-500 p-4 text-center text-white shadow-lg" />
	<div class="custom-layout bar flex h-screen items-center justify-center bg-gray-200 font-bold text-lg" />
	<div class="custom-grid m-6 grid grid-cols-3 gap-4 rounded-md border border-gray-300 p-6 shadow-md" />
	<div class="custom-alert absolute top-0 right-0 m-4 rounded-full bg-red-600 p-2 text-sm text-white" />
	<div class="bar custom-button inline-block rounded border border-green-500 bg-green-300 p-2 text-green-800" />
	<div class="custom-list flex-col space-y-4 divide-y divide-gray-200 rounded-lg bg-white p-6 shadow-md" />
	<div class="custom-background foo relative m-2 h-64 w-full overflow-hidden bg-cover bg-no-repeat" />
	<div class="custom-text foo my-2 font-semibold text-2xl underline" />
	<div class="custom-container bar flex-wrap items-start justify-between bg-purple-200 p-5 text-purple-700" />
	<div class="custom-border gap-8 border-indigo-500 border-l-4 bg-indigo-100 p-3 text-indigo-900" />
</>;

// functions
clsx("foo bar p-4 px-2");
tw`foo bar p-4 px-2`;
tw.div`foo bar p-4 px-2`;
notClassFunction("px-2 foo p-4 bar");
notTemplateFunction`px-2 foo p-4 bar`;
notTemplateFunction.div`px-2 foo p-4 bar`;

// nested values
<div class={"foo bar p-4 px-2"} />;
<div class={["foo bar p-4 px-2"]} />;
<div
	class={{
		"foo bar p-4 px-2": [
			"foo bar p-4 px-2",
			{ "foo bar p-4 px-2": "foo bar p-4 px-2", custom: ["foo bar p-4 px-2"] },
		],
	}}
/>;
clsx(["foo bar p-4 px-2"]);
clsx({
	"foo bar p-4 px-2": [
		"foo bar p-4 px-2",
		{ "foo bar p-4 px-2": "foo bar p-4 px-2", custom: ["foo bar p-4 px-2"] },
	],
});
