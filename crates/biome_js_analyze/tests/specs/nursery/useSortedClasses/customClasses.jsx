// Valid (already sorted)
<div class="container btn-primary px-2 text-red-500 custom-utility" />;

// Invalid (needs sorting - custom classes should go after their layer's classes)
<div class="custom-utility px-2 container text-red-500" />;

// Invalid (utilities need sorting with custom class)
<div class="text-red-500 custom-utility px-2" />;

// Custom component class
<div class="card container block" />;

// Multiple custom classes
<div class="text-lg custom-spacing px-4 custom-layout" />;
