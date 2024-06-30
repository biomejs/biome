<>
	<div onClick={() => void 0} />
	<div onClick={() => void 0} role={undefined} />
	<div onClick={() => void 0} {...props} />
	<div onKeyUp={() => void 0} aria-hidden={false} />
	{/*Static elements; no inherent role */}
	<a onClick={() => void 0} />
	<a onClick={() => {}} />
	<a tabIndex="0" onClick={() => void 0} />
	<area onClick={() => { }} />
	<b onClick={() => {}} />
	<base onClick={() => {}} />
	<bdi onClick={() => {}} />
	<bdo onClick={() => { }} />
	<body onClick={() => { }} />
	<cite onClick={() => { }} />
	<col onClick={() => {}} />
	<colgroup onClick={() => { }} />
	<data onClick={() => {}} />
	<div onClick={() => { }} />
	<head onClick={() => {}} />
	<header onClick={() => {}} />
	<hgroup onClick={() => {}} />
	<i onClick={() => {}} />
	<kbd onClick={() => {}} />
	<link onClick={() => {}} href="#" />
	<map onClick={() => {}} />
	<meta onClick={() => {}} />
	<noscript onClick={() => {}} />
	<object onClick={() => {}} />
	<picture onClick={() => {}} />
	<q onClick={() => {}} />
	<rp onClick={() => {}} />
	<rt onClick={() => {}} />
	<s onClick={() => {}} />
	<samp onClick={() => {}} />
	<script onClick={() => {}} />
	<section onClick={() => {}} />
	<small onClick={() => {}} />
	<source onClick={() => {}} />
	<span onClick={() => {}} />
	<style onClick={() => {}} />
	<title onClick={() => {}} />
	<track onClick={() => {}} />
	<u onClick={() => {}} />
	<var onClick={() => {}} />
	<wbr onClick={() => {}} />
	
	{/* // Handlers */}
	<div onKeyDown={() => {}} />
	<div onKeyPress={() => {}} />
	<div onKeyUp={() => {}} />
	<div onClick={() => {}} />
	<div onMouseDown={() => {}} />
	<div onMouseUp={() => {}} />

	{/* Presentation is a special case role that indicates intentional static semantics */}
	<div role="presentation" onClick={() => {}} />
	<div role="presentation" onKeyDown={() => {}} />
	{/* HTML elements attributed with an abstract role */}
	<div role="command" onClick={() => {}} />
	<div role="composite" onClick={() => {}} />
	<div role="input" onClick={() => {}} />
	<div role="landmark" onClick={() => {}} />
	<div role="range" onClick={() => {}} />
	<div role="roletype" onClick={() => {}} />
	<div role="sectionhead" onClick={() => {}} />
	<div role="select" onClick={() => {}} />
	<div role="structure" onClick={() => {}} />
	<div role="widget" onClick={() => {}} />
	<div role="window" onClick={() => { }} />
	
	{/* <summary> is inherently an interactive element, but in eslint-plugin-jsx-a11y, 
	    it was made non-interactive due to the influence of an external library. */}
	{/* ref: https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/0be7ea95f560c6afc6817d381054d914ebd0b2ca/src/util/isInteractiveElement.js#L86-L89 */}
	{/* <summary onClick={() => {}} /> */}
	
	{/* This element is rejected by HTML Standard */}
	{/* ref: https://lists.w3.org/Archives/Public/public-whatwg-archive/2012Aug/0298.html */}
	{/* <content onClick={() => {}} /> */}

	{/* This element is rejected by HTML Standard */}
	{/* ref: https://html.spec.whatwg.org/multipage/obsolete.html */}
	{/* <acronym onClick={() => {}} /> */}
	{/* <applet onClick={() => {}} /> */}
	{/* <frame onClick={() => { }} /> */}
	{/* <frameset onClick={() => { }} /> */}
	{/* <center onClick={() => {}} /> */}
	{/* <font onClick={() => {}} /> */}	
	{/* <big onClick={() => {}} /> */}
	{/* <blink onClick={() => {}} /> */}
	{/* <rtc onClick={() => {}} /> */}
	{/* <xmp onClick={() => {}} /> */}
	{/* <strike onClick={() => {}} /> */}
	{/* <param onClick={() => {}} /> */}
	{/* <keygen onClick={() => {}} /> */}
	{/* <noembed onClick={() => {}} /> */}
	{/* <spacer onClick={() => {}} /> */}
	{/* <tt onClick={() => {}} /> */}
</>;
