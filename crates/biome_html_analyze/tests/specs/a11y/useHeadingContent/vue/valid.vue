<!-- should not generate diagnostics -->
<template>
  <!-- Heading with content -->
  <h1>heading</h1>
  <h2>Sub heading</h2>

  <!-- Heading with accessible name -->
  <h1 aria-label="Screen reader content"></h1>

  <!-- Heading with nested visible content -->
  <h1><span>visible</span></h1>

  <!-- PascalCase: custom components, should NOT trigger the rule -->
  <H1></H1>
  <MyHeading></MyHeading>

  <!-- Paired PascalCase child: may render accessible content at runtime -->
  <h1><MyComponent></MyComponent></h1>
  <h2><Icon></Icon> label</h2>

  <!-- PascalCase self-closing: custom component, NOT native <img> -->
  <h1><Img /></h1>
</template>
