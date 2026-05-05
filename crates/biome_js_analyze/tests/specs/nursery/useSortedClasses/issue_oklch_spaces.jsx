/* should generate diagnostics */
<div class="border-oklch(0.922 0 0) p-4 m-2" />;

/* should not generate diagnostics */
<div class="m-2 border-oklch(0.922 0 0) p-4" />;

/* should generate diagnostics */
<div className={`border-oklch(0.922 0 0) p-4 m-2`} />;
