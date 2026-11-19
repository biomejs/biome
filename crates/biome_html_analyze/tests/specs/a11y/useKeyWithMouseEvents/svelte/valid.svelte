<!-- should not generate diagnostics -->

<!-- onmouseover paired with onfocus -->
<div onmouseover="handleMouseOver()" onfocus="handleFocus()"></div>

<!-- onmouseout paired with onblur -->
<div onmouseout="handleMouseOut()" onblur="handleBlur()"></div>

<!-- both mouse events with both keyboard events -->
<div onmouseover="handleMouseOver()" onfocus="handleFocus()" onmouseout="handleMouseOut()" onblur="handleBlur()"></div>

<!-- no mouse events -->
<div onclick="handleClick()"></div>

<!-- self-closing elements -->
<input onmouseover="handleMouseOver()" onfocus="handleFocus()" />

<!-- PascalCase custom components should be ignored -->
<MyComponent onmouseover="handleMouseOver()"></MyComponent>
<MyComponent onmouseout="handleMouseOut()"></MyComponent>
<CustomButton onmouseover="handleMouseOver()" onmouseout="handleMouseOut()"></CustomButton>
