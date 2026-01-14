<!-- Native anchors without accessible content should trigger -->
<a></a>
<a>    </a>
<a aria-hidden="true">content</a>
<a aria-hidden>content</a>
<a><span aria-hidden="true">content</span></a>
<a><div aria-hidden="true"><span>nested content</span></div></a>
