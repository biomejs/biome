<!-- should not generate diagnostics -->
<a href="#id">ok</a>
<a href="https://example.com">ok</a>
<a href={somewhere}>ok</a>
<Link href={somewhere}>ok</Link>
<Link href="https://example.com">ok</Link>
<Link href="#">ok</Link>
<a {href}>test</a>
<a {...spread}>test</a>
<a href="#" {...spread}>test</a>
<a href {...spread}>test</a>
<a onclick={something} {...spread}>test</a>
<a href="javascript:void(0)" {...spread}>test</a>
