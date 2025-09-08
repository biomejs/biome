<div className="test" id={someId}>
  {content}
  {showHeader && <h1>Title</h1>}
  <img alt="logo" src={logoSrc} />
  <React.Fragment>
    <span>Inside</span>
  </React.Fragment>
  <custom:element xmlns:custom="http://example.com" {...spreadProps}>
    Text content here
  </custom:element>
</div>
