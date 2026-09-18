<!-- should not generate diagnostics -->
<TestComponent @click="myFunction()" />
<Button @click="myFunction()" />
<div></div>
<div class="foo"></div>
<div class="foo"></div>
<div @click="() => void 0" aria-hidden="true"></div>
<input type="button" @click="() => void 0" />
<input type="button" v-on:click="() => void 0" />
<input type="checkbox" @click="() => void 0" />
<input type="color" @click="() => void 0" />
<input type="date" @click="() => void 0" />
<input type="datetime" @click="() => void 0" />
<input type="datetime-local" @click="() => void 0" />
<input type="email" @click="() => void 0" />
<input type="file" @click="() => void 0" />
<input type="hidden" @click="() => void 0" />
<input type="image" @click="() => void 0" />
<input type="month" @click="() => void 0" />
<input type="number" @click="() => void 0" />
<input type="password" @click="() => void 0" />
<input type="radio" @click="() => void 0" />
<input type="range" @click="() => void 0" />
<input type="reset" @click="() => void 0" />
<input type="search" @click="() => void 0" />
<input type="submit" @click="() => void 0" />
<input type="tel" @click="() => void 0" />
<input type="text" @click="() => void 0" />
<input type="time" @click="() => void 0" />
<input type="url" @click="() => void 0" />
<input type="week" @click="() => void 0" />
<button @click="() => void 0" class="foo"></button>
<datalist @click="() => {}"></datalist>
<option @click="() => void 0" class="foo"></option>
<select @click="() => void 0" class="foo"></select>
<textarea @click="() => void 0" class="foo"></textarea>
<a @click="() => void 0" href="http://x.y.z"></a>
<a @click="() => void 0" href="http://x.y.z" tabindex="0"></a>
<audio @click="() => {}"></audio>
<form @click="() => {}"></form>
<form onsubmit="() => {}"></form>

<div role="button" @click="() => {}"></div>
<div role="checkbox" @click="() => {}"></div>
<div role="combobox" @click="() => {}"></div>
<div role="gridcell" @click="() => {}"></div>
<div role="link" @click="() => {}"></div>
<div role="menuitem" @click="() => {}"></div>
<div role="menuitemcheckbox" @click="() => {}"></div>
<div role="menuitemradio" @click="() => {}"></div>
<div role="option" @click="() => {}"></div>
<div role="radio" @click="() => {}"></div>
<div role="searchbox" @click="() => {}"></div>
<div role="slider" @click="() => {}"></div>
<div role="spinbutton" @click="() => {}"></div>
<div role="switch" @click="() => {}"></div>
<div role="tab" @click="() => {}"></div>
<div role="textbox" @click="() => {}"></div>
<div oncopy="() => {}"></div>
<div oncut="() => {}"></div>
<div onpaste="() => {}"></div>
<div oncompositionend="() => {}"></div>
<div oncompositionstart="() => {}"></div>
<div oncompositionupdate="() => {}"></div>
<div onchange="() => {}"></div>
<div oninput="() => {}"></div>
<div onsubmit="() => {}"></div>
<div onselect="() => {}"></div>
<div ontouchcancel="() => {}"></div>
<div ontouchend="() => {}"></div>
<div ontouchmove="() => {}"></div>
<div ontouchstart="() => {}"></div>
<div onscroll="() => {}"></div>
<div onwheel="() => {}"></div>
<div onabort="() => {}"></div>
<div oncanplay="() => {}"></div>
<div oncanplaythrough="() => {}"></div>
<div ondurationchange="() => {}"></div>
<div onemptied="() => {}"></div>
<div onencrypted="() => {}"></div>
<div onended="() => {}"></div>
<div onerror="() => {}"></div>
<div onloadeddata="() => {}"></div>
<div onloadedmetadata="() => {}"></div>
<div onloadstart="() => {}"></div>
<div onpause="() => {}"></div>
<div onplay="() => {}"></div>
<div onplaying="() => {}"></div>
<div onprogress="() => {}"></div>
<div onratechange="() => {}"></div>
<div onseeked="() => {}"></div>
<div onseeking="() => {}"></div>
<div onstalled="() => {}"></div>
<div onsuspend="() => {}"></div>
<div ontimeupdate="() => {}"></div>
<div onvolumechange="() => {}"></div>
<div onwaiting="() => {}"></div>
<div onload="() => {}"></div>
<div onerror="() => {}"></div>
<div onanimationstart="() => {}"></div>
<div onanimationend="() => {}"></div>
<div onanimationiteration="() => {}"></div>
<div ontransitionend="() => {}"></div>

<!-- HTML elements with an inherent, non-interactive role -->
<address @click="() => {}"></address>
<article @click="() => {}"></article>
<article ondblclick="() => void 0"></article>
<aside @click="() => {}"></aside>
<blockquote @click="() => {}"></blockquote>
<br @click="() => {}" />
<canvas @click="() => {}"></canvas>
<caption @click="() => {}"></caption>
<code @click="() => {}"></code>
<details @click="() => {}"></details>
<dd @click="() => {}"></dd>
<del @click="() => {}"></del>
<dfn @click="() => {}"></dfn>

<dl @click="() => {}"></dl>
<dt @click="() => {}"></dt>
<em @click="() => {}"></em>
<embed @click="() => {}" />
<fieldset @click="() => {}"></fieldset>
<figcaption @click="() => {}"></figcaption>
<figure @click="() => {}"></figure>
<h1 @click="() => {}"></h1>
<h2 @click="() => {}"></h2>
<h3 @click="() => {}"></h3>
<h4 @click="() => {}"></h4>
<h5 @click="() => {}"></h5>
<h6 @click="() => {}"></h6>
<hr @click="() => {}" />
<html @click="() => {}"></html>
<iframe @click="() => {}"></iframe>
<img @click="() => {}" />
<ins @click="() => {}"></ins>
<label @click="() => {}"></label>
<legend @click="() => {}"></legend>
<li @click="() => {}"></li>
<main @click="() => void 0"></main>
<mark @click="() => {}"></mark>
<menu @click="() => {}"></menu>
<meter @click="() => {}"></meter>
<nav @click="() => {}"></nav>
<ol @click="() => {}"></ol>
<optgroup @click="() => {}"></optgroup>
<output @click="() => {}"></output>
<p @click="() => {}"></p>
<pre @click="() => {}"></pre>
<progress @click="() => {}"></progress>
<ruby @click="() => {}"></ruby>
<section @click="() => {}" aria-label="Aa"></section>
<section @click="() => {}" aria-labelledby="js_1"></section>
<strong @click="() => {}"></strong>
<sub @click="() => {}"></sub>
<sup @click="() => {}"></sup>
<table @click="() => {}"></table>
<tbody @click="() => {}"></tbody>
<tfoot @click="() => {}"></tfoot>
<th @click="() => {}"></th>
<thead @click="() => {}"></thead>
<time @click="() => {}"></time>
<tr @click="() => {}"></tr>
<video @click="() => {}"></video>
<ul @click="() => {}"></ul>

<!-- HTML elements attributed with a non-interactive role -->
<div role="listbox" @click="() => {}"></div>
<div role="progressbar" @click="() => {}"></div>
<div role="row" @click="() => {}"></div>
<div role="separator" @click="() => {}"></div>
<div role="scrollbar" @click="() => {}"></div>
<td @click="() => {}"></td>
