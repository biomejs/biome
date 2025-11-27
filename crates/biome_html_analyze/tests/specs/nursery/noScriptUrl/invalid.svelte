<!-- Svelte invalid cases - should trigger the rule -->

<a href="javascript:void(0)">Void</a>
<a href=" javascript:confirm('XSS') ">Confirm</a>
<a href="JAVASCRIPT:doSomething()">Uppercase</a>
