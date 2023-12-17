
useEffect(() => {

}, [a, b])

useMemo(() => {
    return {
        d, e
    }
}, [a, b])

useMemo(() => {

    } // some comment
    ,
    [a, b]
)

useEffect(() => {
    if (clipboardStatus !== "normal") {
        setClipboardStatus("normal");
    }
}, [formatter_ir]);

test.expect(t => {
    t.true(a)
})

test.expect(t => {
    t.true(a)
}, false)

test.something(t => {
    t.true()
}, context => {
    context.flush()
})

// trailing separator omitted
test.expect(t => {
    t.true(a)
}, false,)

test.expect(t => {
    t.true(a)
}, false,
    // comment
    )

// should group start of function expression onto the same line
const Button1 = forwardRef(function Button(props, ref) {
        return <button ref={ref} />;
    }
    );
      
const Button2 = forwardRef(function (props, ref) { return <button ref={ref} />; }
    );

// should break whole call before breaking parameter list
const FilterButton = forwardRefWithLongFunctionName(function FilterButton(props, ref) {
      return <button ref={ref} />;
    }
  );

// Object destructuring should break at exactly the line width. If a conditional
// trailing comma is rendered, this breaks one character too early.
aLongFunctionName(({ parameter1, parameter2, parameter3, parameter4, and }) => {
    const a = 1;
  }
);