<!-- should not generate diagnostics -->
<template>
<article role="presentation"></article>
<div role="button"></div>
<!-- Components should not trigger the rule -->
<MyComponent role="button"></MyComponent>
<Button role="button"></Button>
</template>
