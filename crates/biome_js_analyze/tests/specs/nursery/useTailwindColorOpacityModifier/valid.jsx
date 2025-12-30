/* should not generate diagnostics */
// Valid - using opacity modifier syntax
<div class="bg-red-500/50" />;
<div class="text-blue-600/75" />;
<div class="border-green-400/25" />;

// Valid - no opacity
<div class="bg-red-500" />;
<div class="text-blue-600" />;

// Valid - opacity utility without matching color is unaffected
<div class="bg-opacity-50 text-sm" />;
