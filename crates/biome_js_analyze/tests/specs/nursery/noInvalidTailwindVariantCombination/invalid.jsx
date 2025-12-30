// Invalid - duplicate variants
<div class="hover:hover:bg-red-500" />;
<div class="focus:focus:text-white" />;

// Invalid - conflicting responsive variants
<div class="sm:md:flex" />;
<div class="lg:xl:hidden" />;

// Invalid - mutually exclusive positional variants
<div class="first:last:text-bold" />;
<div class="odd:even:bg-gray-100" />;
