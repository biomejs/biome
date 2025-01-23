// invalid
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

// valid

<>
  <img alt="foo" />
  <img alt={"foo"} />
  <img alt={alt} />
  <img alt="foo" salt={undefined} />
  <img {...this.props} alt="foo" />
  <a />
  <div />
  <img alt={function(e) {} } />
  <div alt={function(e) {} } />
  <img alt={() => void 0} />
  <Img />
  <Component>test</Compoennt>
  <img alt={alt || "Alt text" } />
  <img alt={photo.caption} />;
  <img alt={bar()} />;
  <img alt={foo.bar || ""} />
  <img alt={bar() || ""} />
  <img alt={foo.bar() || ""} />
  <img alt="" />
  <img alt={`${undefined}`} />
  <img alt=" " />
  <img alt="" role="presentation" />
  <img alt="" role="none" />
  <img alt="" role={`presentation`} />
  <img alt="" role={"presentation"} />
  <img alt="this is lit..." role="presentation" />
  <img alt={error ? "not working": "working"} />
  <img alt={undefined ? "working": "not working"} />
  <img alt={plugin.name + " Logo"} />
  <img aria-label="foo" />
  <img aria-labelledby="id1" />
  <img aria-hidden />
  <img aria-hidden={true} />
  <img alt="alt tag" src="" {...(flag && { referrerPolicy: 'no-referrer' })}></img>
  <img src="test" alt="this is a cool image" {...restProps}></img>
  <img alt={alt || ""} {...props} />
</>;
