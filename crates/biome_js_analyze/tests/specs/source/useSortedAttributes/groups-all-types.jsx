// sortScope: "group" with default groups [:IMPLICIT:, :RESERVED:, :DOM_RESERVED:, :REST:, :CALLBACK:]
// Complex components exercising all five group types.

// Unsorted: all five group types mixed
<Hello
  onFocus={handleFocus}
  name="John"
  children={<Avatar />}
  active
  ref={myRef}
  onChange={handleChange}
  lastName="Smith"
  dangerouslySetInnerHTML={{ __html: html }}
  disabled
  key="user-1"
/>;

// Correctly sorted: [:IMPLICIT:, :RESERVED:, :DOM_RESERVED:, :REST:, :CALLBACK:]
// within each group sorted alphabetically
<Hello
  active
  disabled
  key="user-1"
  ref={myRef}
  children={<Avatar />}
  dangerouslySetInnerHTML={{ __html: html }}
  lastName="Smith"
  name="John"
  onChange={handleChange}
  onFocus={handleFocus}
/>;

// Unsorted: RESERVED group — key after REST prop
<Hello name="John" key="1" />;

// Unsorted: DOM_RESERVED group — children before REST props (should follow RESERVED)
<Hello name="John" key="1" children={<Foo />} />;

// Unsorted: multiple callbacks out of alphabetical order, after implicit props
<Hello onFocus={fn} onClick={fn} active name="John" />;

// Unsorted with spread fence: each segment covers all group types
<Hello
  onClick={handleClick}
  name="John"
  disabled
  key="user-1"
  children={<Avatar />}
  {...defaults}
  onFocus={handleFocus}
  lastName="Smith"
  active
  ref={myRef}
  dangerouslySetInnerHTML={{ __html: html }}
/>;
