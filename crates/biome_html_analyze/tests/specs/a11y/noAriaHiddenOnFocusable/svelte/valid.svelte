<!-- should not generate diagnostics -->
<!-- Custom components should not trigger the rule -->
<MyButton aria-hidden="true">click</MyButton>
<CustomLink aria-hidden="true">link</CustomLink>
<AppInput aria-hidden="true" />

<!-- Non-focusable native elements with aria-hidden -->
<div aria-hidden="true"></div>
<span aria-hidden="true"></span>

<!-- Focusable but tabindex="-1" removes from tab order -->
<button aria-hidden="true" tabindex="-1"></button>
<div tabindex="-1" aria-hidden="true"></div>

<!-- input type="hidden" is not focusable -->
<input type="hidden" aria-hidden="true" />

<!-- a without href is not focusable -->
<a aria-hidden="true">no href, not focusable</a>

<!-- contenteditable="false" is not focusable -->
<div contenteditable="false" aria-hidden="true">not editable</div>

<!-- area without href is not focusable -->
<area aria-hidden="true" />
