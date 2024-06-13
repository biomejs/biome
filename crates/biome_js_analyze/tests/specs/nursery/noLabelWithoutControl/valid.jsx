// Always valid
<div />;
<CustomElement />;
<input type="hidden" />;

// For element
<label for="js_id"><span><span><span>A label</span></span></span></label>;
<label for="js_id" aria-label="A label" />;
<label for="js_id" aria-labelledby="A label" />;
<label htmlFor="js_id"><span><span><span>A label</span></span></span></label>;
<label htmlFor="js_id" aria-label="A label" />;
<label htmlFor="js_id" aria-labelledby="A label" />;

// Nesting valid
<label>A label<input /></label>;
<label>A label<textarea /></label>;
<label><img alt="A label" /><input /></label>;
<label><img aria-label="A label" /><input /></label>;
<label><span>A label<input /></span></label>;
<label><span><span>A label<input /></span></span></label>;
<label><span><span><span>A label<input /></span></span></span></label>;
<label><span><span><span><span>A label</span><input /></span></span></span></label>;
<label><span><span><span><span aria-label="A label" /><input /></span></span></span></label>;
<label><span><span><span><input aria-label="A label" /></span></span></span></label>;

// Other controls
<label>foo<meter /></label>;
<label>foo<output /></label>;
<label>foo<progress /></label>;
<label>foo<textarea /></label>;
