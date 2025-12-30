// Invalid: leading whitespace
<div class="  flex p-4" />;

// Invalid: trailing whitespace
<div class="flex p-4  " />;

// Invalid: multiple consecutive spaces
<div class="flex    p-4" />;

// Invalid: leading, trailing and multiple spaces
<div class="  flex    p-4   bg-white  " />;

// Invalid: only whitespace differences
<div className="  hover:bg-blue-500   " />;

// Invalid: newlines and tabs (should be normalized)
<div class="flex
    p-4" />;
