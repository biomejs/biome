/* should not generate diagnostics */

// Valid: no duplicates
<div class="flex p-4" />;
<div class="p-4 text-red-500 bg-white" />;
<div className="hover:bg-blue-500 focus:bg-blue-500" />;

// Valid: similar but different classes
<div class="p-4 px-4 py-4" />;
<div class="text-red-500 text-lg" />;

// Valid: empty class
<div class="" />;

// Valid: single class
<div class="flex" />;
