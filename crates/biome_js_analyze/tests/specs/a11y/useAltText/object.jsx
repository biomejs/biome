// invalid

<>
  <object />
  <object><div aria-hidden /></object>
  <object title={undefined} />
  <object aria-label="" />
  <object aria-labelledby="" />
  <object aria-label={undefined} />
  <object aria-labelledby={undefined} />
</>;

//valid

<>
  <object aria-label="foo" />
  <object aria-labelledby="id1" />
  <object>Foo</object>
  <object><p>This is descriptive!</p></object>
  <Object />
  <object title="An object" />
</>;
