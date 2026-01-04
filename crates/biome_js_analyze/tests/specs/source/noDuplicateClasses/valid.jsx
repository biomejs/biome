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

// Valid: single-quoted JSX strings
<div class='flex p-4' />;

// Valid: template literals without duplicates
<div className={`flex p-4 ${condition}`} />;
<div className={`${prefix} p-4 m-2 ${suffix}`} />;

// Valid: utility function calls without duplicates
cn("flex p-4 m-2");
clsx("bg-red-500 text-white");

// Valid: object member names without duplicates
cn({ "flex p-4": isActive });

// Valid: whitespace-only (no classes to duplicate)
<div class="   " />;

// Valid: case-sensitive (CSS classes are case-sensitive)
<div class="Flex flex" />;
