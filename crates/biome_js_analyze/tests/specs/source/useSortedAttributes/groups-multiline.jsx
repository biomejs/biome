// sortScope: "group" with :MULTILINE: group at the end
// Multiline props should sort after all other non-group props

// Unsorted: multiline before regular props
<Hello
  style={{
    color: "red",
  }}
  name="John"
/>;

// Unsorted: multiline before callback
<Hello
  onClick={
    () => doSomething()
  }
  name="John"
  onChange={fn}
/>;

// Correctly ordered: regular props first, then multiline
<Hello name="John" style={{
  color: "red",
}} />;
