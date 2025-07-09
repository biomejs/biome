<div className="container" />

<label htmlFor="input" />

<div className="container" htmlFor="input" />

<div className={`container ${isActive ? 'active' : ''}`} />

<label htmlFor={`input-${id}`} />

<MyComponent className="custom-class" />

<MyLabel htmlFor="my-input" />

<div {...props} className="additional-class" />

<label {...labelProps} htmlFor="spread-input" />

<div>
  <span className="nested-class" />
</div>

<form>
  <label htmlFor="nested-input" />
</form>

<div className="class1 class2 class3" />

<label htmlFor={getInputId()} />

{isVisible && <div className="conditional-class" />}

{hasLabel && <label htmlFor="conditional-input" />}

{items.map(item => <div key={item.id} className="item-class" />)}

{inputs.map(input => <label key={input.id} htmlFor={input.id} />)} 