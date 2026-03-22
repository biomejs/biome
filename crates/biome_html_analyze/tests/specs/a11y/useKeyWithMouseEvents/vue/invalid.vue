<template>
  <!-- should generate diagnostics -->

  <!-- onmouseover without onfocus -->
  <div onmouseover="handleMouseOver()"></div>

  <!-- onmouseout without onblur -->
  <div onmouseout="handleMouseOut()"></div>

  <!-- has wrong keyboard pair -->
  <div onmouseover="handleMouseOver()" onblur="handleBlur()"></div>
  <div onmouseout="handleMouseOut()" onfocus="handleFocus()"></div>

  <!-- self-closing elements -->
  <input onmouseover="handleMouseOver()" />

  <!-- different element types -->
  <a onmouseover="handleMouseOver()">link</a>
  <button onmouseout="handleMouseOut()">button</button>
</template>
