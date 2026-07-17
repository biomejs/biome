<!-- Newlines around expression should be preserved -->
<div>
	{name}
</div>

<!-- No newlines = stays inline -->
<div>{name}</div>

<!-- Newlines around expression + comment should be preserved -->
<div>
	{name}<!-- comment -->
</div>

<!-- No newlines around expression + comment = stays inline -->
<div>{name}<!-- comment --></div>

<!-- Multiple expressions with newlines -->
<div>
	{foo}
	{bar}
</div>

<!-- Newlines around expression in inline element should be preserved -->
<span class="video-length">
	{lengthDisplay}
</span>
<span class="video-length">{lengthDisplay}</span>

<!-- Text in inline element should still collapse -->
<span>
	hello
	world
</span>

<!-- Mixed inline content should stay inline -->
<span>before {value} after</span>

<!-- Mixed multiline inline content should stay multiline -->
<span>
	hello
	<em>x</em>
</span>
