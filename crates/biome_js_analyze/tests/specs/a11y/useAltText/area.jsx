// invalid

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

//valid

<>
  <area aria-label="foo" />
  <area aria-labelledby="id1" />
  <area aria-hidden />
  <area aria-hidden={true} />
  <area alt="" />
  <area alt="This is descriptive!" />
  <area alt={altText} />
  <Area />
</>;
