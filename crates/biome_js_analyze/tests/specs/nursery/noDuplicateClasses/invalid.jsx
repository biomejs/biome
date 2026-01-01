// Invalid: exact duplicates
<div class="flex flex" />;
<div class="p-4 text-red-500 p-4 bg-white" />;
<div className="hover:bg-blue-500 hover:bg-blue-500" />;

// Invalid: multiple duplicates
<div class="flex p-4 flex m-2 p-4" />;

// Invalid: duplicates with complex classes
<div class="hover:focus:m-2 foo hover:focus:m-2" />;

// Invalid: duplicates in the middle
<div class="text-lg text-center text-lg font-bold" />;

// Invalid: duplicates with arbitrary values
<div class="w-[100px] p-4 w-[100px]" />;

// Invalid: single-quoted JSX strings (should preserve quotes)
<div class='mt-4 mt-4' />;

// Invalid: template literals (should preserve boundary spaces)
<div className={`flex flex ${condition}`} />;
<div className={`${prefix} p-4 p-4 ${suffix}`} />;

// Invalid: duplicates with newlines (should collapse whitespace)
<div class="foo
bar
foo" />;

// Invalid: duplicates with tabs
<div class="flex	p-4	flex" />;
