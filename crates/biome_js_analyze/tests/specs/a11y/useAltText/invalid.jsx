/* should generate diagnostics */
<>
	<area />
	<area alt />
	<area alt={undefined} />
	<area src="xyz" />
	<area {...this.props} />
	<area aria-label="" />
	<area aria-label={undefined} />
	<area aria-labelledby="" />
	<area aria-labelledby={undefined} />
	<area aria-hidden={false} />
	<area aria-hidden={undefined} />
</>;

<>
	<img />
	<img alt />
	<img alt={undefined} />
	<img src="xyz" />
	<img role />
	<img {...this.props} />
	<img alt={undefined} role="presentation" />
	<img alt role="presentation" />
	<img role="presentation" />
	<img role="none" />
	<img aria-label={undefined} />
	<img aria-labelledby={undefined} />
	<img aria-hidden={undefined} />
	<img aria-label="" />
	<img aria-labelledby="" />
	<img aria-hidden={false} />
</>;

<>
	<input type="image" />
	<input type="image" alt />
	<input type="image" alt={undefined} />
	<input type="image">Foo</input>
	<input type="image" {...this.props} />
	<input type="image" aria-label="" />
	<input type="image" aria-label={undefined} />
	<input type="image" aria-labelledby="" />
	<input type="image" aria-labelledby={undefined} />
	<input type="image" aria-hidden={false} />
	<input type="image" aria-hidden={undefined} />
</>;

<>
	<object />
	<object><div aria-hidden /></object>
	<object title={undefined} />
	<object aria-label="" />
	<object aria-labelledby="" />
	<object aria-hidden={false} />
	<object aria-label={undefined} />
	<object aria-labelledby={undefined} />
	<object aria-hidden={undefined} />
</>;
