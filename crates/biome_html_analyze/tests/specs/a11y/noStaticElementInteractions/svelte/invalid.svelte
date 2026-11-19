<!-- should generate diagnostics -->
<div onclick={() => void 0}></div>
<div onclick={() => void 0} role={undefined}></div>
<div onclick={() => void 0} {...props}></div>
<div onkeyup={() => void 0} aria-hidden={false}></div>

<!-- Static elements; no inherent role -->
<a onclick={() => void 0}></a>
<a onclick={() => {}}></a>
<a tabindex="0" onclick={() => void 0}></a>
<area onclick={() => {}} />
<b onclick={() => {}}></b>
<base onclick={() => {}} />
<bdi onclick={() => {}}></bdi>
<bdo onclick={() => {}}></bdo>
<body onclick={() => {}}></body>
<cite onclick={() => {}}></cite>
<col onclick={() => {}} />
<colgroup onclick={() => {}}></colgroup>
<data onclick={() => {}}></data>
<div onclick={() => {}}></div>
<head onclick={() => {}}></head>
<header onclick={() => {}}></header>
<hgroup onclick={() => {}}></hgroup>
<i onclick={() => {}}></i>
<kbd onclick={() => {}}></kbd>

<map onclick={() => {}}></map>
<meta onclick={() => {}} />
<noscript onclick={() => {}}></noscript>
<object onclick={() => {}}></object>
<picture onclick={() => {}}></picture>
<q onclick={() => {}}></q>
<rp onclick={() => {}}></rp>
<rt onclick={() => {}}></rt>
<s onclick={() => {}}></s>
<samp onclick={() => {}}></samp>
<section onclick={() => {}}></section>
<small onclick={() => {}}></small>
<source onclick={() => {}} />
<span onclick={() => {}}></span>
<title onclick={() => {}}></title>
<track onclick={() => {}} />
<u onclick={() => {}}></u>
<var onclick={() => {}}></var>
<wbr onclick={() => {}} />

<!-- Handlers -->
<div onkeydown={() => {}}></div>
<div onkeypress={() => {}}></div>
<div onkeyup={() => {}}></div>
<div onclick={() => {}}></div>
<div onmousedown={() => {}}></div>
<div onmouseup={() => {}}></div>

<!-- Presentation is a special case role that indicates intentional static semantics -->
<div role="presentation" onclick={() => {}}></div>
<div role="presentation" onkeydown={() => {}}></div>

<!-- HTML elements attributed with an abstract role -->
<div role="command" onclick={() => {}}></div>
<div role="composite" onclick={() => {}}></div>
<div role="input" onclick={() => {}}></div>
<div role="landmark" onclick={() => {}}></div>
<div role="range" onclick={() => {}}></div>
<div role="roletype" onclick={() => {}}></div>
<div role="sectionhead" onclick={() => {}}></div>
<div role="select" onclick={() => {}}></div>
<div role="structure" onclick={() => {}}></div>
<div role="widget" onclick={() => {}}></div>
<div role="window" onclick={() => {}}></div>
