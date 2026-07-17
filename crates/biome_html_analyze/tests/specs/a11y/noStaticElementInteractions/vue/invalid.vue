<!-- should generate diagnostics -->
<div @click="() => void 0"></div>
<div v-on:click="() => void 0"></div>
<div onkeyup="() => void 0" aria-hidden="false"></div>

<!-- Static elements; no inherent role -->
<a @click="() => void 0"></a>
<a @click="() => {}"></a>
<a tabindex="0" @click="() => void 0"></a>
<area @click="() => {}" />
<b @click="() => {}"></b>
<base @click="() => {}" />
<bdi @click="() => {}"></bdi>
<bdo @click="() => {}"></bdo>
<body @click="() => {}"></body>
<cite @click="() => {}"></cite>
<col @click="() => {}" />
<colgroup @click="() => {}"></colgroup>
<data @click="() => {}"></data>
<div @click="() => {}"></div>
<head @click="() => {}"></head>
<header @click="() => {}"></header>
<hgroup @click="() => {}"></hgroup>
<i @click="() => {}"></i>
<kbd @click="() => {}"></kbd>

<map @click="() => {}"></map>
<meta @click="() => {}" />
<noscript @click="() => {}"></noscript>
<object @click="() => {}"></object>
<picture @click="() => {}"></picture>
<q @click="() => {}"></q>
<rp @click="() => {}"></rp>
<rt @click="() => {}"></rt>
<s @click="() => {}"></s>
<samp @click="() => {}"></samp>
<script @click="() => {}"></script>
<section @click="() => {}"></section>
<small @click="() => {}"></small>
<source @click="() => {}" />
<span @click="() => {}"></span>
<style @click="() => {}"></style>
<title @click="() => {}"></title>
<track @click="() => {}" />
<u @click="() => {}"></u>
<var @click="() => {}"></var>
<wbr @click="() => {}" />

<!-- Handlers -->
<div onkeydown="() => {}"></div>
<div onkeypress="() => {}"></div>
<div onkeyup="() => {}"></div>
<div @click="() => {}"></div>
<div onmousedown="() => {}"></div>
<div onmouseup="() => {}"></div>

<!-- Presentation is a special case role that indicates intentional static semantics -->
<div role="presentation" @click="() => {}"></div>
<div role="presentation" onkeydown="() => {}"></div>

<!-- HTML elements attributed with an abstract role -->
<div role="command" @click="() => {}"></div>
<div role="composite" @click="() => {}"></div>
<div role="input" @click="() => {}"></div>
<div role="landmark" @click="() => {}"></div>
<div role="range" @click="() => {}"></div>
<div role="roletype" @click="() => {}"></div>
<div role="sectionhead" @click="() => {}"></div>
<div role="select" @click="() => {}"></div>
<div role="structure" @click="() => {}"></div>
<div role="widget" @click="() => {}"></div>
<div role="window" @click="() => {}"></div>
