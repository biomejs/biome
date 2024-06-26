<>
	{/* attributes */}
	<div class="foo bar p-4 px-2" />
	<div className="foo bar p-4 px-2" />
	<div customClassAttribute="px-2 foo p-4 bar" />
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
	{/* variant sorting */}
	<div class="custom-style1 foo p-4 shadow-lg checked:text-center hover:bg-blue-500 focus:text-white hover:focus:rounded-lg" />
	<div class="custom-layout bar flex items-center font-bold first-letter:text-lg required:justify-center valid:h-screen valid:required:bg-gray-200" />
	<div class="custom-grid m-6 grid grid-cols-3 gap-4 p-6 optional:rounded-md focus-within:border focus-within:border-gray-300 focus-within:hover:shadow-md" />
	<div class="lg:custom-alert sm:top-0 sm:rounded-full sm:bg-red-600 sm:text-white md:right-0 md:p-2 lg:absolute lg:m-4 xl:text-sm" />
	<div class="bar custom-button rounded border p-2 checked:sm:inline-block sm:checked:border-green-500 hover:md:bg-green-300 hover:lg:text-green-800" />
	{/* TODO: arbitrary variant */}
	<div class="container py-4 text-red-500 focus:bg-red-100 hover:focus:bg-red-200 group-hover:flex-col has-[:visited]:flex print:text-red-50 [&nth-child(2)]:text-red-300 [&nth-child(3)]:text-red-200  [&nth-child(2)]:[&nth-child(6)]:text-red-200" />
	<div class="py-4 text-black checked:visited:bg-yellow-300 hover:text-red-400 focus:bg-sky-100 group-first:text-yellow-400 [&nth-child(1)]:group-first:text-red-100 [&nth-child(2)]:text-black [&nth-child(2)]:focus:text-red-100 [&nth-child(2)]:hover:focus:bg-red-900" />
	<div class="group-target:font-bold group-[:visited]:text-red-400 group-aria-disabled:bg-red-50" />
	<div class="text-red-400 has-[:checked]:focus:bg-yellow-300 group-has-[.custom-class]:focus:underline aria-[sort=ascending]:bg-red-300 group-aria-[sort=ascending]:text-yellow-200" />
</>;

// functions
clsx("px-2 foo p-4 bar");
tw`px-2 foo p-4 bar`;
tw.div`px-2 foo p-4 bar`;
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
clsx(["px-2 foo p-4 bar"]);
clsx({
	"px-2 foo p-4 bar": [
		"px-2 foo p-4 bar",
		{ "px-2 foo p-4 bar": "px-2 foo p-4 bar", custom: ["px-2 foo p-4 bar"] },
	],
});
