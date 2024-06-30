// invalid

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

//valid

<>
  <object aria-label="foo" />
  <object aria-labelledby="id1" />
  <object aria-hidden />
  <object aria-hidden={true} />
  <object>Foo</object>
  <object><p>This is descriptive!</p></object>
  <Object />
  <object title="An object" />
</>;
