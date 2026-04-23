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
