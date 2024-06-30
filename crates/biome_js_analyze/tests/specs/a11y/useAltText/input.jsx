// invalid
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

// valid

<>
  <input />
  <input type="foo" />
  <input type="image" aria-label="foo" />
  <input type="image" aria-labelledby="id1" />
  <input type="image" aria-hidden />
  <input type="image" aria-hidden={true} />
  <input type="image" alt="" />
  <input type="image" alt="This is descriptive!" />
  <input type="image" alt={altText} />
  <Input type="image" />
</>;
