
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