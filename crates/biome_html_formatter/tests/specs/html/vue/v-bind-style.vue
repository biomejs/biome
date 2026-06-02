<div :style="$$$$foo">
	This style attribute should not be treated as a css embedded block, and should be left as is.
	The $ is there to trip up the css parser, if it gets invoked (which is not the desired behavior).
</div>
