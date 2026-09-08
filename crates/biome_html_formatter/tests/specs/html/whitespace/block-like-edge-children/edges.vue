<template>
  <video src="brave.webm">
    <track kind="subtitles" src="brave.en.vtt" />
    <source src="brave.webm" />
  </video>
  <object>
    <param name="movie" value="movie.mp4" />
    <span>fallback</span>
  </object>
</template>

<template><video src="brave.webm"><track kind="subtitles" src="brave.en.vtt"/><source src="brave.webm"/></video><object><param name="movie" value="movie.mp4"/><span>fallback</span></object></template>

<template><span><button type="button">inline-block start</button>following text that makes the parent wrap</span><span>preceding text that makes the parent wrap<button type="button">inline-block end</button></span></template>

<template><object><section>block-like last child with enough text to force its parent element to break across lines</section></object><span>touching sibling</span></template>

<template><p>block first</p><div>block last</div></template>
