<template>
<!-- should generate diagnostics -->

<!-- div with onclick but no keyboard handler -->
<div onclick="handleClick()"></div>

<!-- span with onclick but no keyboard handler -->
<span onclick="handleClick()">text</span>

<!-- <a> without href is not interactive -->
<a onclick="handleClick()">bare link</a>
</template>
